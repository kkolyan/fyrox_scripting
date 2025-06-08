# Fyrox Lite

## About a project
Make games with Rust-written [Fyrox Game Engine](https://fyrox.rs) without writing code in Rust.

###  [Fyrox / C#](https://kkolyan.github.io/fyrox_lite/sdk_cs/)
Prebuilt [Fyrox editor](https://fyrox-book.github.io/beginning/editor_overview.html) imbued with [C#](https://learnxinyminutes.com/csharp/) scripting support.
* Requires [.Net 8.0](https://dotnet.microsoft.com/en-us/download/dotnet/thank-you/sdk-8.0.410-windows-x64-installer).
* Run and compile game from either Fyrox Editor, any IDE (with debug support) or command line.

### [Fyrox / Lua](https://kkolyan.github.io/fyrox_lite/sdk_lua/)
Prebuilt [Fyrox editor](https://fyrox-book.github.io/beginning/editor_overview.html) imbued with [Lua](https://learnxinyminutes.com/lua/) scripting support.
* No additional SDKs required.
* Run game from either Fyrox Editor or command line.

### Current state
* It works (see [Examples](#examples))
* The [subset of supported Fyrox API](https://kkolyan.github.io/fyrox_lite/sdk_cs/scripting_api.html) is pretty limited.
* It probably lacks polishing
* Prebuilt binaries temporary unavailable (due to high pace of development)

### Supported platforms
Currently, Windows only. MacOSX and Linux support will be soon. WASM and mobile support is in agenda, but has low priority.

## Examples
There is a simple game that written in different scripting languages to demonstrate the part of Fyrox API currently exposed to scripting languages.
* [in Fyrox / C#](https://github.com/kkolyan/fyrox_lite/blob/main/showcase/guards_cs)
* [in Fyrox / Lua](https://github.com/kkolyan/fyrox_lite/blob/main/showcase/guards_lua)
* [in Fyrox / Rust (vanilla)](https://github.com/kkolyan/fyrox_lite/blob/main/showcase/guards_vanilla)

## Credits
* Special thanks to [Dmitry Stepanov](https://github.com/mrDIMAS), author of Fyrox and other [Fyrox](https://github.com/FyroxEngine/Fyrox/) Contributors for such a powerful and inspiring foundation.
* Grateful acknowledgment to the [Godot Engine](https://github.com/godotengine/godot) contributors for Godot's modular design and liberal license, allowed to reuse their Math classes in this project.
* Sincere thanks to the open-source community for sharing tools, ideas and knowledge.

## Contributing
See the [contributions guidelines](https://github.com/kkolyan/fyrox_lite/blob/main/CONTRIBUTING.md) for more info.

## Feedback
Any feedback is extremely appreciated.
Feel free to contact me in Discord `kkolyan_true` ([kkolyan_true](https://discord.com/users/333644000302989314)) or just [create issue](https://github.com/kkolyan/fyrox_lite/issues/new).
