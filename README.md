## About a project
Make games with [Fyrox](https://fyrox.rs) without a writing code in Rust.

### Fyrox / C#
Prebuilt [Fyrox editor](https://fyrox-book.github.io/beginning/editor_overview.html) imbued with [C#](https://learnxinyminutes.com/csharp/) scripting support.
* Requires [.Net 8.0](https://dotnet.microsoft.com/en-us/download/dotnet/thank-you/sdk-8.0.410-windows-x64-installer).
* Run and compile game from either Fyrox Editor or any IDE (with debug support)

[Fyrox C# SDK Guide](guide_cs.md)

### Fyrox / Lua
Prebuilt [Fyrox editor](https://fyrox-book.github.io/beginning/editor_overview.html) imbued with [Lua](https://learnxinyminutes.com/lua/) scripting support.
* No additional SDKs required.
* Run game from either Fyrox Editor or command line.

[Fyrox Lua SDK Guide](guide_lua.md)

### Supported platforms
Currently, I test project only on Windows. Fyrox [supports a lot of platforms](https://fyrox-book.github.io/introduction/requirements.html), so that's short-term limitation. Editor and game support for MacOSX and Linux support will be soon.

### Simplified API
Scripting languages access Fyrox via a simplified API.

## Examples
There is a simple game that written in different scripting languages to demonstrate the part of Fyrox API currently exposed to scripting languages.
* [in Fyrox / C#](showcase/guards_cs)
* [in Fyrox / Lua](showcase/guards_lua)
* [in Fyrox / Rust (vanilla)](showcase/guards_vanilla)

## Credits
* Special thanks to [Dmitry Stepanov](https://github.com/mrDIMAS), author of Fyrox and other [Fyrox](https://github.com/FyroxEngine/Fyrox/) Contributors for such a powerful and inspiring foundation.
* Grateful acknowledgment to the [Godot Engine](https://github.com/godotengine/godot) contributors for Godot's modular design and liberal license, allowed to reuse their Math classes in this project.
* Sincere thanks to the open-source community for sharing tools, ideas and knowledge.

## Contributing
See the [contributions guidelines](CONTRIBUTING.md) for more info.

## Feedback
Any feedback is extremely appreciated.
Feel free to contact me in Discord `kkolyan_true` ([kkolyan_true](https://discord.com/users/333644000302989314)) or just [create issue](https://github.com/kkolyan/fyrox_lite/issues/new).
