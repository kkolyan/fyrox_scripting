- [Overview](#overview)
- [Demo](#demo)
- [For users (who make Games)](#for-users-who-make-games)
	- [Vision](#vision)
	- [Current state](#current-state)
	- [How to use it now](#how-to-use-it-now)
	- [How to write scripts in Lua](#how-to-write-scripts-in-lua)
- [For contributors](#for-contributors)
	- [Lite API](#lite-api)
	- [Contract](#contract)
	- [Language Implementations](#language-implementations)
	- [Lua Implementation](#lua-implementation)
- [Feedback](#feedback)

## Overview
[First-class](# "&quot;First-class&quot; means it's designed to allow one to make games with Fyrox without seeing a single Rust line of code. The principle is the same as Godot, Unity or UE allow to make games with GDSript, C# or Blueprints") `C#` and `Lua` scripting support for [Fyrox Engine](https://github.com/FyroxEngine/Fyrox). 

The project also provides a framework to maintain multiple languages implementation at the cost of one. Engine features are integrated via a language-agnostic Lite API, making them available to all supported scripting languages.

## Demo
There is a game that written in different scripting languages to demonstrate the currently Lua-exposed subset of Fyrox API.
* [demo game in Lua](lua/examples/guards)
* [the same demo game in C#](cs/examples/Guards)
* [the same demo reference implementation in Rust (without Fyrox Lite)](https://github.com/kkolyan/fyrox_guards)

## For users (who make Games)

### Vision
When this project is released, this is how games should be made:
1. Download pre-built `Fyrox Lite Toolkit` for a $Lang.
2. Make models/images/sounds using 3rd party tools like Blender.
3. Make scripts using any suitable text editor or IDE.
4. Use the `Fyrox Editor $Lang` (from the Fyrox Lite Toolkit) to create scenes and attach models, images, sound and scripts from the project directory to scene objects.
5. Run a game using a button in Fyrox Editor.
6. Run a game using a `Fyrox Executor $Lang` (from the Fyrox Lite Toolkit), if you like to play without editor.

### Current state
1. Subset of exposed Fyrox API is pretty limited: input, messages, working with scene graph, prefab instantiation, basic physics, basic UI Text. Check out [Lua Annotations](lua/annotations) or  for details. Though, that's already enough for gameplay prototyping.
2. There is only `Lua` and `C#` languages support currently.
3. Hot-reload supported for Lua. In-editor hot-reload enabled by default (affects list of scripts and available fields in inspector). In-game hot-reload allows to write a game literally when playing it, but it's fundamentally error-prone and requires a skill to be used with convenience, so it's disabled by default and can be enabled with LuaPlugin constructor parameter. Hot-reload check is triggered when window is switched back to editor (or game, if enabled).
4. There is no existing pre-built toolkit yet, so editor and executor should be run from the source code (which is pretty easy actually - see instruction below).
5. There are a lot of temporary limitations, decribed in [known_issues.md](known_issues.md).
 
### How to use it now (Lua)
1. install Rust (https://www.rust-lang.org/tools/install)
2. checkout Fyrox Lite `git clone --recursive https://github.com/kkolyan/fyrox_lite_lua` to some directory (let's call it `$FYROX_LITE_HOME`).
3. let's call a directory with your game project files a `$GAME_PROJECT`.
4. open terminal in this directory (`cd $GAME_PROJECT`). That's important - otherwise Fyrox will not be able to find the resources attached to scene objects.
5. Run editor: `cargo run --release -p editor-lua --manifest-path $FYROX_LITE_HOME/Cargo.toml`.
6. Run game without editor: `cargo run --release -p executor-lua --manifest-path $FYROX_LITE_HOME/Cargo.toml`.

### How to write scripts in Lua
Check out [demo game](lua/examples/guards).

There are two kind of scripts:
1. Node scripts (for instance [Bullet.lua](lua/examples/guards/scripts/Bullet.lua)), that in general replicates [Fyrox Scripts](https://fyrox-book.github.io/scripting/script.html). They can be attached to nodes in scene editor and configured via inspector.
2. Global scripts (for instance [Game.lua](lua/examples/guards/scripts/Game.lua)). These scripts purpose to load scene initially and share global state between node scripts. It is somewhat close to [Fyrox Plugin](https://fyrox-book.github.io/scripting/plugin.html), but without technical things like scripts registration.

The [Lua Annotations](lua/annotations), besides being documentation of sorts, also proides intellisense for VSCode (code completion, type checking). Be sure they are in scope of your project for this to work. Note that even though annotations are Lua files, they are not supposed to be executed, so do not place them in `scripts` directory.

## For contributors

### Lite API
Lite API is a Rust library that provides a scripting-language-friendly facade over the Fyrox API. It isn't bound to a specific language, but it's design assumes that scripting language has GC and some kind of OOP.

This library is supposed to be updated frequently when it's necessary to expose some part of Fyrox API to scripting language. Package [fyrox-lite](fyrox-lite) is the place where most of changes to be done. 

Exposed API should comply with the rules. Following types allowed (owned only, no references allowed):
* primitives (limited set of them, for the sake of simplicity)
* `data types` - `#[fyrox_lite]`-annotated structures or enums. they have copy-on-asign semantic. It's supposed that on the scripting language side they are represented in its native data structures. That's not allowed to expose Rust methods of this structures - all necessary methods should be provided by the language specific implementation.
* `engine types` - defined by annotating non-trait `impl`s with this same `#[fyrox_lite]` attribute. Script code can invoke exposed methods (using `ffi` or analogs), but internal structure of this types is completely hidden. Script code can instantiate an engine type only if there is exposed method for this. Handles are clonable and clone operation only clones the handle, not the underlying object. If underlying object has limited lifecycle, then it should provide the methods to deal with it.
* predefined abstract types. That's a family of traits, expected to be implemented by every language provider. they are not intended to be changed frequently. The central type is [UserScript](fyrox-lite/src/spi.rs).
* `Vec<T>`, `Option<T>`, `Result<T>` where `T` is allowed type..

Note that Vector3 and Quaternion for Lua are of an `engine type`, but for some languages (C# for instance) they probably would be a `data type`, because language-native implementation of vector arithmetics could be more efficient than `ffi` to `nalgebra`. That's why nalgebra-backed types are in [fyrox-lite-math](fyrox-lite-math) and [fyrox-lite](fyrox-lite) exposes methods with shallow math structs instead of nalgebra-backed ones.

`#[fyrox_lite]` attrubute is not just a marker - it provides almost complete realtime enforcement of this rules.

### Contract
There is a [metadata model](lite-model/src/lib.rs) that serves as contract between `Lite API` and `Language Implementation`s. There is the package `lite-parser` that is responsible for collecting metadata using this same `#[lite_api]` attribute. For the debug purposes, collected metadata is dumped in json ([fyrox-lite](fyrox-lite/src/domain.json), [fyrox-lite-math](fyrox-lite-math/src/domain.json)).

### Language Implementations
There is no specific rules for this, but it's supposed that language implementation consumes the Lite API metadata and produces a Rust code with Fyrox `Plugin` implementation that loads scripts metadata (script names, property types and names), allowing attaching them in inspector, and provides a runtime for a target scripting language.

### Lua Implementation
* `lua/fyrox-lua` - the runtime library, provides [LuaPlugin](lua/fyrox-lua/src/fyrox_lua_plugin.rs) and [ExternalScriptProxy](lua/fyrox-lua/src/external_script_proxy.rs). [mlua](https://github.com/mlua-rs/mlua) crate used to embed Lua. LuaU interpreter is choosen (mlua allow to switch them easily) just because it was easiest to compile on Windows, but there is no dependency on specific interpreter features.
* `lua/editor-lua` / `lua/executor-lua` - desktop instantiations of previously mentioned `LuaPlugin`.
* `lua/luagen-lib` - dynamic part. It uses Lite API metadata to generate both [Lua bindings](lua/fyrox-lua/src/generated) and [Lua annotations](lua/annotations). Currently, `luagen-lib` is not integrated with build and invoked with `cargo run --bin luagen` ([code](tools/src/bin/luagen.rs)).

## Feedback
Any feedback is extremely appreciated.
Feel free to contact me in Discord `kkolyan_true` ([kkolyan_true](https://discord.com/users/333644000302989314)) or just [create issue](https://github.com/kkolyan/fyrox_lite_lua/issues/new).
