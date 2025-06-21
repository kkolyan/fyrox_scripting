# Backlog

* [x] editor can be launched with the project directory as argument
* [x] project can be opened using UI dialog
    * tried to do it using windows native UI, but that's non-cross-platform and actually pretty bad (haven't managed to
      choose folder using Save dialog)
    * let's just implement dialog using Fyrox. It'll be crossplatform and flexible. Though, probably Look'n'feel should
      be improved to not scare newcomers with non-typical look
    * Actually, there are crates for this. Done
* [x] refresh of script metadata when DLL recompiled
    * [x] fucking with watcher - it seems to lock assembly file (not certainly), and it doesn't detect changes
    * [x] seems like it's C# who locked assembly (even after unloading). solved by loading assembly from text.
    * [x] UUID added to global scripts. Guards editor doesn't load scripts now
        * for C# they UUID for global scripts is not added. it's present in code, generated each time during scanning
          scripts, but doesn't need to be defined by user, because, there is no hot reload support in game mode for C#
    * [x] Lua part is not compilable
* [x] C# project is runnable from Rider
* [x] Run game using button in Editor (for Lua that's primary launch option, for C# - for designers, because devs can
  launch it from IDE)
    * probably support of custom run/build profiles
    * for C#, seems like built-in profile system is enough, because `dotnet` command is sufficient for running game, but
      actually it's not, because:
        * (not blocker) it's prohibited to have zero build commands
        * Fyrox ignores profile name in "settings.ron", should it ignore it if profile set from code if default Build
          Settings made configurable?
* [x] get rid of useless cargo-based profiles in editor
    * add to Fyrox option to hide or customize default build/run profiles
* [x] make Lua editor run handy
    * [x] command line argument to project path
    * [x] dialog window to choose folder
* [x] check if shader warnings are somehow related to the C# integration
    * these warnings are the same as in vanilla, so ignore them
* [x] make Lua scripts in playmode hot-reload on focus change only
* [x] Add "generate IDE project" button for C#
    * do not need. let's just generated them anyway
* [x] make delays between checking DLL existence (avoid spam "failed to initialize watcher")
    * we don't need to check it. just do not open editor until DLL initially not loaded
* [x] discuss with mrDIMAS allowing to build project when scene is not opened
    * just fixed
* [x] copy or link lua annotations to the opened project
* [x] messages should be identified somehow
* [x] editor maintains UUIDs for scripts somehow (directly in "cs" files, or in "meta" files)
    * I don't like idea to generate it externally due to the mess with Rider keeping code in memory for some time
    * [x] let's create plugin for Rider! ChatGPT says that's easy
* [x] add info about ScriptMessagePayload derive to Fyrox book
* [x] Add "open IDE" button for C#
    * rejected. feature is more non-trivial then useful.
* [x] rustdoc rendered to Lua annotations
* [x] rustdoc rendered to C# facade
* [x] move GlobalScript out of Plugin section
* [x] get rid of `lite_` in `[GlobalScript](lite_plugin/../Plugin/GlobalScript.md)`
* [x] fix rendering of sections so they don't look like C# namespaces
* [x] fix Color location for Lua
* [x] Macos
* [x] Linux
    * checked on WSLg. the only flaw is slowness and inability to detect raw mouse motion. Thats's obviously side effect
      of WSL
* [x] fix Color class for C# (probably pick it from Godot too)
* [x] add icons to an executables
* [x] change editor icon?
* [x] fix warnings
* [x] update to .Net 9.0
* [x] refine the case when user misconfigured C# scripts
    * [x] missing UUID attribute
    * [x] missing default constructor
    * [x] readonly fields
* [x] refine Lua script error handling
* [x] make C# project compiled on editor start regardless of assembly presence
* [ ] investigate Lua binary size. probably, mlua code style gives this. Use raw bindings?
* [ ] allow user "retry" when opening project failed with script loading error
* [ ] fix C# compilation warnings
* [ ] Crosshair!
* [ ] implement "Open in IDE" button
* [ ] self-contained game export for
* [ ] downloadable via GitHub releases
* [ ] describe plans about custom rust code
    * [ ] Fyrox Lite as a crate for usual Fyrox project
    * [ ] Rust modules for usual Fyrox Lite project
