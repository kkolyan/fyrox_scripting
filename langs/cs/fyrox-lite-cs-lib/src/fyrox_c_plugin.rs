use crate::arena::Arena;
use crate::auto_dispose::AutoDispose;
use crate::c_lang::UnpackedObject;
use crate::errors::ResultTcrateLangSpecificErrorExt;
use crate::external_script_proxy::invoke_callback;
use crate::external_script_proxy::ExternalScriptProxy;
use crate::lazy_watcher::LazyWatcher;
use crate::scripted_app::APP;
use fyrox::core::log::Log;
use fyrox::core::notify::EventKind;
use fyrox::core::reflect::prelude::*;
use fyrox::core::reflect::Reflect;
use fyrox::core::visitor::prelude::*;
use fyrox::core::visitor::Visit;
use fyrox::core::watcher::FileSystemWatcher;
use fyrox::event::Event;
use fyrox::plugin::DynamicPlugin;
use fyrox::plugin::Plugin;
use fyrox::plugin::PluginContext;
use fyrox::plugin::PluginRegistrationContext;
use fyrox::script::constructor::ScriptConstructor;
use fyrox::script::Script;
use fyrox_lite::lite_input::Input;
use fyrox_lite::script_metadata::{ScriptDefinition, ScriptKind};
use fyrox_lite::script_object::NodeScriptObject;
use fyrox_lite::script_object_residence::ScriptResidence;
use fyrox_lite::wrapper_reflect;
use std::cell::{Cell, RefCell};
use std::fmt::Debug;
use std::ops::DerefMut;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use fyrox_lite::global_script_object::ScriptObject;
use fyrox_lite::global_script_object_residence::GlobalScriptResidence;
use crate::external_global_script_proxy::ExternalGlobalScriptProxy;

#[derive(Visit, Reflect)]
pub struct CPlugin {
    #[visit(skip)]
    #[reflect(hidden)]
    pub failed: bool,

    #[visit(skip)]
    #[reflect(hidden)]
    pub need_reload: Cell<bool>,

    pub scripts: RefCell<GlobalScriptList>,

    #[visit(skip)]
    #[reflect(hidden)]
    pub hot_reload: HotReload,
}

pub enum HotReload {
    Disabled,
    Enabled { watcher: RefCell<LazyWatcher> },
}

impl Debug for CPlugin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExternalScriptPlugin")
            .field("failed", &self.failed)
            .field("scripts", &self.scripts)
            .finish()
    }
}

impl CPlugin {
    pub fn new(reloadable_assembly_path: Option<PathBuf>) -> Self {
        Self {
            failed: false,
            need_reload: Default::default(),
            scripts: RefCell::new(Default::default()),
            hot_reload: reloadable_assembly_path
                .map(|path| {
                    println!("trying to initialize watcher for file: {}", path.to_str().unwrap());
                    HotReload::Enabled {
                        watcher: RefCell::new(LazyWatcher::TryingToInitialize {
                            path,
                            duration: Duration::from_millis(500),
                            last_checked_at: SystemTime::UNIX_EPOCH,
                        })
                    }
                })
                .unwrap_or(HotReload::Disabled),
        }
    }

    pub fn is_candidate_for_reload(&self) -> bool {
        let HotReload::Enabled { watcher } = &self.hot_reload else {
            return false;
        };
        let mut watcher = watcher.borrow_mut();
        let watcher = watcher.deref_mut();
        if let LazyWatcher::TryingToInitialize { path, duration, last_checked_at } = watcher {
            let last_check_sec_ago = last_checked_at.elapsed()
                .map(|it| it.as_secs_f32())
                .unwrap_or_else(|it| -it.duration().as_secs_f32());
            if last_check_sec_ago < 3.0 {
                return false;
            }
            *last_checked_at = SystemTime::now();
            let w = FileSystemWatcher::new(&path, *duration);
            if let Ok(w) = w {
                println!("assembly file detected, creating watcher: {}", path.to_str().unwrap());
                *watcher = LazyWatcher::Initialized(w);
                return true;
            } else {
                println!("failed to initialize watcher for path {}: {:?}", &path.display(), w.err().unwrap());
            }
            return false;
        }
        let LazyWatcher::Initialized(watcher) = watcher else {
            return false;
        };
        let mut reload = false;
        while let Some(event) = watcher.try_get_event() {
            Log::info(format!("FS watcher event: {event:?}"));
            match &event.kind {
                EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_) => {
                    println!("assembly file change detected");
                    reload = true;
                }
                _ => {}
            }
        }
        reload
    }
}

impl Default for CPlugin {
    fn default() -> Self {
        todo!()
    }
}

impl Clone for CPlugin {
    fn clone(&self) -> Self {
        todo!()
    }
}

impl Plugin for CPlugin {
    fn register(&self, context: PluginRegistrationContext) {
        APP.with_borrow_mut(|app| {
            let app = app.as_mut().unwrap();
            app.load_scripts_metadata();
        });
        APP.with_borrow(|app| {
            let app = app.as_ref().unwrap();
            for md in app.scripts_metadata.as_ref().unwrap().node_scripts.values() {
                let def = Arc::new(ScriptDefinition {
                    metadata: md.md.clone(),
                    assembly_name: self.assembly_name(),
                });
                let name = def.metadata.class.clone();
                let class = md.id;

                println!("adding script constructor {}: {}", &name, md.md.uuid);
                context
                    .serialization_context
                    .script_constructors
                    .add_custom(
                        md.md.uuid,
                        ScriptConstructor {
                            name: name.to_string(),
                            source_path: "",
                            assembly_name: self.assembly_name(),
                            constructor: Box::new(move || {
                                Script::new(ExternalScriptProxy {
                                    name: name.to_string(),
                                    class,
                                    data: ScriptResidence::Packed(NodeScriptObject::new(&def)),
                                })
                            }),
                        },
                    )
                    .unwrap();
            }
            for md in app.scripts_metadata.as_ref().unwrap().global_scripts.values() {
                let mut plugin_scripts = self.scripts.borrow_mut();
                let def = Arc::new(ScriptDefinition {
                    metadata: md.md.clone(),
                    assembly_name: self.assembly_name(),
                });
                let name = def.metadata.class.clone();
                let class = md.id;
                plugin_scripts.inner_mut().push(ExternalGlobalScriptProxy {
                    name: name.to_string(),
                    class,
                    data: GlobalScriptResidence::Packed(ScriptObject::new(&def)),
                });
            }
        });
    }

    fn init(&mut self, scene_path: Option<&str>, mut context: PluginContext) {
        Input::init_thread_local_state();
        for script in self.scripts.borrow_mut().0.iter_mut() {
            script.data.ensure_unpacked(&mut self.failed);
            invoke_callback(&mut context, |app| {
                let scene_path = scene_path.map(|it| it.to_string()).into();
                let id = script.data.inner_unpacked().unwrap().instance.inner();
                let result = (app.functions.on_game_init)(id, scene_path);
                result.into_result().handle_scripting_error();
            });
        }
    }

    fn update(&mut self, context: &mut PluginContext) {
        for script in self.scripts.borrow_mut().0.iter_mut() {
            script.data.ensure_unpacked(&mut self.failed);
            invoke_callback(context, |app| {
                (app.functions.on_game_update)(script.data.inner_unpacked().unwrap().instance.inner()).into_result().handle_scripting_error();
            });
        }
    }

    fn on_os_event(&mut self, event: &Event<()>, _context: PluginContext) {
        Input::on_os_event(event);
    }

    fn post_update(&mut self, _context: &mut PluginContext) {
        Input::post_fixed_update();
        Arena::free();
    }
}

#[derive(Debug, Default, Clone)]
pub struct GlobalScriptList(Vec<ExternalGlobalScriptProxy>);

impl GlobalScriptList {
    pub fn inner(&self) -> &Vec<ExternalGlobalScriptProxy> {
        &self.0
    }
    pub fn inner_mut(&mut self) -> &mut Vec<ExternalGlobalScriptProxy> {
        &mut self.0
    }
}

impl Visit for GlobalScriptList {
    fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
        let mut guard = visitor.enter_region(name)?;

        for item in self.inner_mut().iter_mut() {
            item.data.visit(item.name.as_str(), &mut guard)?;
        }
        Ok(())
    }
}

impl Reflect for GlobalScriptList {
    wrapper_reflect! {0}
}

impl DynamicPlugin for CPlugin {
    fn display_name(&self) -> String {
        format!("C# Plugin")
    }

    fn is_reload_needed_now(&self) -> bool {
        if !self.need_reload.get() {
            self.need_reload.set(self.is_candidate_for_reload());
        }
        self.need_reload.get()
    }

    fn as_loaded_ref(&self) -> &dyn Plugin {
        self
    }

    fn as_loaded_mut(&mut self) -> &mut dyn Plugin {
        self
    }

    fn is_loaded(&self) -> bool {
        true
    }

    fn prepare_to_reload(&mut self) {
        Log::info("prepare_to_reload");
        self.need_reload.set(false);
    }

    fn reload(
        &mut self,
        fill_and_register: &mut dyn FnMut(&mut dyn Plugin) -> Result<(), String>,
    ) -> Result<(), String> {
        Log::info("Reloading C# scripts");
        self.scripts.borrow_mut().inner_mut().clear();
        fill_and_register(self)
    }
}
