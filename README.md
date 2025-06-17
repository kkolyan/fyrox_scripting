<p align="center">
  <a href="https://fyrox.rs/">
    <img src="https://github.com/FyroxEngine/Fyrox/blob/master/pics/logo.png?raw=true" width="64" height="64" alt="Fyrox"/>&#8203;
  </a>
  <a href="https://www.lua.org/about.html">
    <img src="http://www.rozek.de/Lua/Lua-Logo_128x128.png" width="64" height="64" alt="Lua" />&#8203;
  </a>
  <a href="https://learn.microsoft.com/en-us/dotnet/csharp/tour-of-csharp/overview">
    <img src="https://github.com/dotnet/vscode-csharp/blob/main/images/csharpIcon.png?raw=true" width="64" height="64" alt="C#" />&#8203;
  </a>
</p>

[![License (MIT)](https://img.shields.io/badge/license-MIT-blue)](https://github.com/kkolyan/fyrox_lite/blob/master/LICENSE)
[![docs.rs](https://img.shields.io/badge/docs%3A%20C%23-website-blue)](https://kkolyan.github.io/fyrox_lite/fyrox_cs/index.html)
[![docs.rs](https://img.shields.io/badge/docs%3A%20Lua-website-blue)](https://kkolyan.github.io/fyrox_lite/fyrox_lua/index.html)
[![docs.rs](https://img.shields.io/badge/JetBrains%20Marketplace-website-blue)](https://plugins.jetbrains.com/plugin/27613-fyroxlite?noRedirect=true)

## Fyrox Lite

Create games with Rust-based [Fyrox Game Engine](https://fyrox.rs) without coding in Rust.

### [Fyrox C# ⤴](https://kkolyan.github.io/fyrox_lite/fyrox_cs/index.html)

Prebuilt [Fyrox editor](https://fyrox-book.github.io/beginning/editor_overview.html) imbued
with [C#](https://learnxinyminutes.com/csharp/) scripting support.

* Requires [.Net 9.0 SDK](https://dotnet.microsoft.com/en-us/download/dotnet)
  for development.
* Run and compile game from either Fyrox Editor, any IDE (with debug support) or command line.
* Optional [JB Rider plugin](https://plugins.jetbrains.com/plugin/27613-fyroxlite?noRedirect=true).

### [Fyrox Lua ⤴](https://kkolyan.github.io/fyrox_lite/fyrox_lua/index.html)

Prebuilt [Fyrox editor](https://fyrox-book.github.io/beginning/editor_overview.html) imbued
with [Lua](https://learnxinyminutes.com/lua/) scripting support.

* No additional SDKs required.
* Run game from either Fyrox Editor or command line.

## Current state

* It works (see [Examples](#examples))
* The [subset of supported Fyrox API](https://kkolyan.github.io/fyrox_lite/fyrox_cs/scripting_api.html) is pretty
  limited.
* It probably lacks polishing
* Prebuilt binaries temporary unavailable (due to high pace of development)

## Supported platforms

| Platform    | Edit scripts & scenes | Lua: Play | C#: Play |
|-------------|-----------------------|-----------|----------|
| Windows     | ✅                     | ✅         | ✅        |
| macOS       | ✅                     | ✅         | ✅        |
| Linux       | ✅                     | ✅         | ✅        |
| iOS         | ❌                     | 🟡        | 🟠       |
| Android     | ❌                     | 🟡        | 🟠       |
| WebAssembly | ❌                     | 🟡        | 🔴       |

* ✅ Supported
* 🟡 Relatively easy to support; likely if requested
* 🟠 Feasible but more complex; likely if broadly requested
* 🔴 Possible with significant effort; likely if widely requested
* ❌ No plans to support

## Examples

There is a simple game that written in different scripting languages to demonstrate the part of Fyrox API currently
exposed to scripting languages.

* [in Fyrox C#](https://github.com/kkolyan/fyrox_lite/blob/main/showcase/guards_cs)
* [in Fyrox Lua](https://github.com/kkolyan/fyrox_lite/blob/main/showcase/guards_lua)
* [in Fyrox / Rust (vanilla)](https://github.com/kkolyan/fyrox_lite/blob/main/showcase/guards_vanilla)

## Credits

* Special thanks to [Dmitry Stepanov](https://github.com/mrDIMAS), author of Fyrox, and
  other [Fyrox](https://github.com/FyroxEngine/Fyrox/) Contributors for such a powerful and inspiring foundation.
* Gratefully acknowledge the Godot Engine contributors for its modular design and permissive license, which made it
  possible
  to [incorporate portions of its C# code into this project](https://github.com/kkolyan/fyrox_lite/tree/main/langs/cs/fyrox-lite-sln/fyrox_lite_cs_netcore/src/FromGodot).
* Sincere thanks to the open-source community for sharing tools, ideas and knowledge.

## Contributing

See the [contributions guidelines](https://github.com/kkolyan/fyrox_lite/blob/main/CONTRIBUTING.md) for more info.

## Feedback

Any feedback is extremely appreciated.
Feel free to contact me directly in Discord
`kkolyan_true` ([kkolyan_true](https://discord.com/users/333644000302989314)), or
tag me in [Fyrox Discord](https://discord.gg/xENF5Uh).
Or [create issue](https://github.com/kkolyan/fyrox_lite/issues/new).
