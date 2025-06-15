# Dependencies

| Use case                              | .Net SDK 9.0 | Rust SDK |
|---------------------------------------|--------------|----------|
| Build Fyrox C# SDK from sources       | Required     | Required |
| Develop scripts & scenes              | Required     | -        |
| Quick Play                            | Required     | -        |
| Standalone Play (not implemented yet) | -            | -        |

# How to Install

Currently, it needs to be built from sources. Examples are for Windows, for other platforms just skip ".exe" suffix and
use OS-specific paths, and it will work.

1. Install [.Net SDK 9.0](https://dotnet.microsoft.com/en-us/download/dotnet)
2. Install [Rust](https://rustup.rs/). It's needed only to build SDK.
3. (skip for non-Windows OS-es) Install [Git for Windows](https://git-scm.com/downloads). This is needed for the
   `Git Bash` terminal - required to build the SDK on Windows using shell scripts.
4. Run `install_sdk_cs.sh /c/dev/fyrox_cs_sdk` in `Git Bash` terminal. Note: the path must not exist before
   installation (to prevent mistakes).

# How to use

Get Fyrox C# SDK.

Install [.NET 9.0 SDK](https://dotnet.microsoft.com/en-us/download/dotnet) if it's not installed yet.

Select a project directory where you'll keep scripts and other resources. Double-click on
`C:/dev/fyrox_cs_sdk/fyroxed_cs_netcore.exe` and choose that directory.

Minimal C# project files will be created automatically and C# project will be automatically compiled.

Game can be launched using Play button in editor. Also, game can be launched using IDE with debug support (run profile
for JB Rider generated, for other IDE note that working directory should be the project directory, not the build
directory).

## Scripts

Inherit classes from `NodeScript` to create scripts that can be attached to scene nodes in editor via dropdown on
Inspector panel at the right.

Inherit from `GlobalScript` to create singleton scripts (no need for `[Uuid(...)]` attribute).

NodeScript classes should be annotatee using `[Uuid(...)]`
attribute.
Consider [JB Rider plugin for Fyrox C#](https://plugins.jetbrains.com/plugin/27613-fyroxlite?noRedirect=true) - it
generates UUID, but also puts attribute if script created via context menu, also it highlight fields. If you use
different
IDE,
UUID
value [using any online service like this](https://www.uuidgenerator.net/).

Look into [guards_cs](https://github.com/kkolyan/fyrox_lite/tree/main/showcase/guards_cs) project as an example.

## Scripts Reloading

Editor reloads scripts metadata automatically after their compilation. In-game hot reload is not supported.

## Scenes & Assets

Original Fyrox engine is 100% relevant in all non-code things, so Fyrox Book is advised for non-code information.

* [Fyrox Book: Editor Overview](https://fyrox-book.github.io/beginning/editor_overview.html)
* [Fyrox Book: Assets](https://fyrox-book.github.io/beginning/assets.html)

## CLI

The `fyroxed_cs_netcore.exe` can also be invoked via terminal with project path argument.
