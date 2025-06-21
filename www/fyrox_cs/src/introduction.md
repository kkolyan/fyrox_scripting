# Dependencies

.Net SDK 9.0 is required for development.

Currently, it's also required on player machine. That's temporary limitation, self-contained distribution will be
supported soon.

# Download

* [Windows](https://github.com/kkolyan/fyrox_lite_bin/raw/refs/heads/0.1.0/fyrox_csharp-0.1.0-win.zip)
* [macOS](https://github.com/kkolyan/fyrox_lite_bin/raw/refs/heads/0.1.0/fyrox_csharp-0.1.0-macos.zip)
    * Binaries are not signed, so it requires to allow it running on your Mac in macOS settings.
* [Linux](https://github.com/kkolyan/fyrox_lite_bin/raw/refs/heads/0.1.0/fyrox_csharp-0.1.0-linux.zip)
* [Install from sources](#install-from-sources)

# How to use

Install [.NET 9.0 SDK](https://dotnet.microsoft.com/en-us/download/dotnet) if it's not installed yet.

Select a project directory where you'll keep scripts and other resources.

Run editor:

* (All OS) Specify project directory via command line argument `C:/dev/fyrox_cs_sdk/fyroxed_cs_netcore.exe <path>`
* (Windows only) Double-click on `C:/dev/fyrox_cs_sdk/fyroxed_cs_netcore.exe` icon and choose project directory in
  opened
  dialog window.
* (Windows only) Drop project directory on the `C:/dev/fyrox_cs_sdk/fyroxed_cs_netcore.exe` icon in file explorer.

Minimal C# project files will be created automatically and C# project will be automatically compiled.

Game can be launched using Play button in editor. Also, game can be launched using IDE with debug support (run profile
for JB Rider generated, for other IDE note that working directory should be the project directory, not the build
directory). Also, game can be launched using `dotnet run` in the game directory.

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

# Install from sources

That's optional, because prebuilt binaries [are available](#download).

1. Install [.NET 9.0 SDK](https://dotnet.microsoft.com/en-us/download/dotnet) if it's not installed yet.
2. Install latest [Rust](https://rustup.rs/) toolchain.
3. (Windows only) Install [Git Bash](https://git-scm.com/downloads/win). Other MSYS2 distribution haven't tested.
4. Checkout sources and install using shell command (use Git Bash on windows)
    ```shell 
    git clone https://github.com/kkolyan/fyrox_lite.git \
      && cd fyrox_lite \
      && chmod +x **/*.sh \
      && ./bash/cs_install_sdk.sh <installation path>
   ```

`<installation path>` now contains ready-to-use binaries.
