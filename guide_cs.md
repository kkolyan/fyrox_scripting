# Fyrox C# SDK

# How to Install
Currently, it needs to be built from sources and only on Windows (temporary limitation).

1. Install [.Net 8.0](https://dotnet.microsoft.com/en-us/download/dotnet/thank-you/sdk-8.0.410-windows-x64-installer) (later versions probably will work fine too, but not tested)
2. Install [Rust](https://rustup.rs/). It's needed only to build SDK.
3. Install [Git for Windows](https://git-scm.com/downloads). This is needed for the `Git Bash` terminal - required to build the SDK on Windows using shell scripts.
4. Run `install_sdk_cs.sh /c/dev/fyrox_cs_sdk` in `Git Bash` terminal. Note: the path must not exist before installation (to prevent mistakes).

# How to use

Install [.Net 8.0](https://dotnet.microsoft.com/en-us/download/dotnet/thank-you/sdk-8.0.410-windows-x64-installer) (later versions probably will work fine too, but not tested).

Select a project directory where you'll keep scripts and other resources. Double-click on `C:/dev/fyrox_cs_sdk/fyroxed_cs_netcore.exe` and choose that directory. 

Minimal C# project files will be created automatically and C# project will be automatically compiled.

Game can be launched using Play button in editor. Also, game can be launched using IDE with debug support (run profile for JB Rider generated, for other IDE note that working directory should be the project directory, not the build directory).

## Scripts
Inherit classes from `NodeScript` to create scripts that can be attached to scene nodes in editor via dropdown on Inspector panel at the right. Annotate them with using `[Uuid(...)]` attribute, value [can be generated here](https://www.uuidgenerator.net/).

Inherit from `GlobalScript` to create singleton scripts (no need for `[Uuid(...)]` attribute).

For details about `Fyrox Lite API`, look into [guards_cs](showcase/guards_cs) project as a reference.

## Scripts Reloading
Editor reloads scripts metadata automatically after their compilation. In-game hot reload is not supported.

## Scenes & Assets
* [Fyrox Book: Editor Overview](https://fyrox-book.github.io/beginning/editor_overview.html)
* [Fyrox Book: Assets](https://fyrox-book.github.io/beginning/assets.html)

## CLI
The `fyroxed_cs_netcore.exe` can also be invoked via terminal with project path argument.
