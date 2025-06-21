# Dependencies

No dependencies

# Download

* [Windows](https://github.com/kkolyan/fyrox_lite_bin/raw/refs/heads/0.1.0/fyrox_lua-0.1.0-win.zip)
* [macOS](https://github.com/kkolyan/fyrox_lite_bin/raw/refs/heads/0.1.0/fyrox_lua-0.1.0-macos.zip)
    * Binaries are not signed, so it requires to allow it running on your Mac in macOS settings.
* [Linux](https://github.com/kkolyan/fyrox_lite_bin/raw/refs/heads/0.1.0/fyrox_lua-0.1.0-linux.zip)
* [Install from sources](#install-from-sources)

# How to use

* (All OS) Specify that directory via command line argument `C:/dev/fyrox_lua_sdk/fyroxed_lua.exe <path>`
* (Windows only) Double-click on `C:/dev/fyrox_lua_sdk/fyroxed_lua.exe` icon and choose that directory in opened
  dialog window.
* (Windows only) Drop that directory on the `C:/dev/fyrox_lua_sdk/fyroxed_lua.exe` icon in file explorer.

Game can be launched using Play button in editor. Also, game can be launched using
`C:/dev/fyrox_lua_sdk/fyrox_lite_lua.exe c:/dev/my_lua_game` command.

## Scripts

To emulate some OOP concepts, [Lua Annotaions](https://luals.github.io/wiki/annotations/) are used.

Following code defines a `MyScript` class, inherited from `Script`

```lua
---@uuid 3e0d5f2b-6f9b-4c9a-a4fb-7cda5fae9d8d
---@class MyScript : Script
---@field velocity Vector3
---@field collider Node
---@field power number
MyScript = script_class()

function MyScript:on_update(dt)
end
```

Defined fields are accessible in editor UI and saved into scene files.

Inherit classes from `NodeScript` to create scripts that can be attached to scene nodes in editor via dropdown on
Inspector
panel at the right. for `@uuid` annotation value [can be generated here](https://www.uuidgenerator.net/).

Inherit from `GlobalScript` to create singleton scripts. Unlike C# scripts, Lua global scripts require UUID.

Look into [guards_cs](https://github.com/kkolyan/fyrox_lite/tree/main/showcase/guards_lua) project as an example.

## Scripts Reloading

Editor reloads scripts metadata automatically after their change and window re-focusing. In-game hot reload works the
same way.

## Scenes & Assets

* [Fyrox Book: Editor Overview](https://fyrox-book.github.io/beginning/editor_overview.html)
* [Fyrox Book: Assets](https://fyrox-book.github.io/beginning/assets.html)

## CLI

The `fyroxed_lua.exe` can also be invoked via terminal with project path argument.

# Install from sources

That's optional, because prebuilt binaries [are available](#download).

1. Install latest [Rust](https://rustup.rs/) toolchain.
2. (Windows only) Install [Git Bash](https://git-scm.com/downloads/win). Other MSYS2 distribution haven't tested.
3. Checkout sources and install using shell command (use Git Bash on windows)
    ```shell 
    git clone https://github.com/kkolyan/fyrox_lite.git \
      && cd fyrox_lite \
      && chmod +x **/*.sh \
      && ./bash/lua_install_sdk.sh <installation path>
   ```

`<installation path>` now contains ready-to-use binaries.
