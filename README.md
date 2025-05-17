- [Overview](#overview)
- [Demo](#demo)
- [For users (who make Games)](#for-users-who-make-games)
	- [Vision](#vision)
	- [Current state](#current-state)
	- [How to use it now](#how-to-use-it-now)
	- [How to write scripts in Lua](#how-to-write-scripts-in-lua)
- [For contributors](#for-contributors)
- [Feedback](#feedback)

## Overview
First-class `C#` and `Lua` scripting support for [Fyrox Engine](https://github.com/FyroxEngine/Fyrox).

Make games in scripts only without touching Rust - just like Unity (C#), Godot (GDScript), UE (Blueprints).

`Lua` doesn't require anything but prebuilt binary of `Fyrox/Lua`. `C#` also requires to install Net Core for development, but distributed game is self-contained (in progress, AOT is intended to be used).

This project also provides a framework to maintain multiple languages implementation at the cost of one. Engine features are integrated via a language-agnostic `Lite API`, making them available to all supported scripting languages.

## Demo
There is a game that written in different scripting languages to demonstrate the currently Lua-exposed subset of Fyrox API.
* [demo game in Lua](lua/examples/guards)
* [the same demo game in C#](cs/examples/Guards)
* [the same demo reference implementation in Rust (without Lite API)](https://github.com/kkolyan/fyrox_guards)

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

[CONTRIBUTING.md](CONTRIBUTING.md)

## Feedback
Any feedback is extremely appreciated.
Feel free to contact me in Discord `kkolyan_true` ([kkolyan_true](https://discord.com/users/333644000302989314)) or just [create issue](https://github.com/kkolyan/fyrox_lite_lua/issues/new).
