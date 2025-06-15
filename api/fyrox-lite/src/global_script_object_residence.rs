use crate::global_script_object::ScriptObject;
use crate::script_object::Lang;
use fyrox::core::log::Log;
use fyrox::core::visitor::Visit;
use fyrox::core::visitor::VisitResult;
use fyrox::core::visitor::Visitor;
use fyrox::core::Uuid;
use std::fmt::Debug;
use std::fmt::Formatter;

/// Initially, when script is loaded from file (scene or save game), it's in "packed" mode.
/// First time this script receives `on_update` callback, it's converted to "unpacked", by
/// transfering state into UserData managed by Lua VM. Thoughm serialization should work fine,
/// because Visit is implemented in both modes.
pub enum GlobalScriptResidence<T: Lang> {
    Packed(ScriptObject<T>),
    Unpacked(T::UnpackedGlobalScriptObject),
}

impl<T: Lang> GlobalScriptResidence<T> {
    pub fn is_packed(&self) -> bool {
        match self {
            GlobalScriptResidence::Packed(_) => true,
            GlobalScriptResidence::Unpacked(_) => false,
        }
    }

    pub fn inner_unpacked(
        self: &GlobalScriptResidence<T>,
    ) -> Option<&T::UnpackedGlobalScriptObject> {
        match self {
            GlobalScriptResidence::Packed(_it) => None,
            GlobalScriptResidence::Unpacked(it) => Some(it),
        }
    }

    pub fn ensure_unpacked(self: &mut GlobalScriptResidence<T>, failed: &mut bool) {
        if *failed {
            // don't spam logs, though, plugin is completely broken at this point
            return;
        }
        if self.is_packed() {
            // script was just loaded from the scene file or safe game. unpack it!
            let data = match self {
                GlobalScriptResidence::Packed(it) => {
                    let so = T::unpack_global_script(it);
                    match so {
                        Ok(it) => it,
                        Err(err) => {
                            Log::err(format!("failed to unpack global script: {:?}", err));
                            *failed = true;
                            return;
                        }
                    }
                }
                GlobalScriptResidence::Unpacked(_) => panic!("WTF?"),
            };
            *self = Self::Unpacked(data);
        }
    }

    pub fn with_script_object<R>(&self, f: impl FnOnce(&ScriptObject<T>) -> R) -> R {
        match self {
            GlobalScriptResidence::Packed(it) => f(it),
            GlobalScriptResidence::Unpacked(it) => todo!(),
            // ScriptResidence::Unpacked(it) => f(&it.borrow().unwrap()),
        }
    }

    pub fn with_script_object_mut<R>(&mut self, f: impl FnOnce(&mut ScriptObject<T>) -> R) -> R {
        match self {
            GlobalScriptResidence::Packed(it) => f(it),
            GlobalScriptResidence::Unpacked(it) => todo!(),
            // ScriptResidence::Unpacked(it) => f(&mut it.borrow_mut().unwrap()),
        }
    }

    pub fn id(&self) -> Uuid {
        match self {
            GlobalScriptResidence::Packed(it) => uuid_of_script(it),
            GlobalScriptResidence::Unpacked(it) => T::id_of_global(it),
        }
    }
}

pub fn uuid_of_script<T: Lang>(script: &ScriptObject<T>) -> Uuid {
    script.def.metadata.uuid
}

impl<T: Lang> Debug for GlobalScriptResidence<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GlobalScriptResidence::Packed(it) => it.fmt(f),
            GlobalScriptResidence::Unpacked(it) => write!(f, "Packed( {:?} )", it),
        }
    }
}

impl<T: Lang> Clone for GlobalScriptResidence<T> {
    fn clone(&self) -> Self {
        match self {
            GlobalScriptResidence::Packed(it) => GlobalScriptResidence::Packed(it.clone()),

            // will implement when know when cloning is really needed during game cycle
            GlobalScriptResidence::Unpacked(_) => {
                panic!("cloning for Lua-backed ScriptData is not supported")
            }
        }
    }
}

impl<T: Lang> Drop for GlobalScriptResidence<T> {
    fn drop(&mut self) {
        match self {
            GlobalScriptResidence::Packed(_it) => {
                // ScriptObject is dropped automatically without delay
            }
            GlobalScriptResidence::Unpacked(it) => {
                T::drop_script_object_to_prevent_delayed_destructor_global(it);
            }
        }
    }
}

impl<T: Lang> Visit for GlobalScriptResidence<T> {
    fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
        match self {
            GlobalScriptResidence::Packed(it) => it.visit(name, visitor),
            GlobalScriptResidence::Unpacked(it) => it.visit(name, visitor),
        }
    }
}
