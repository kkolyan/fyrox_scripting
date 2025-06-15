# Dependencies
| Use case                          | Rust SDK |
|-----------------------------------|----------|
| Build Fyrox / C# SDK from sources | Required |
| Develop scripts & scenes          | -        |
| Play                              | -        |

# How to Install
Currently, you need to build from source. Examples use .exe for Windows; on other platforms, remove the suffix and use platform-specific paths—it will work the same.

1. Install [Rust](https://rustup.rs/). It's needed only to build SDK.
2. (skip for non-Windows OS-es) Install [Git for Windows](https://git-scm.com/downloads). This is needed for the `Git Bash` terminal - required to build the SDK on Windows using shell scripts.
3. Run `install_sdk_lua.sh /c/dev/fyrox_lua_sdk` in `Git Bash` terminal. Note: the path must not exist before installation (to prevent mistakes).

# How to use

Select a project directory where you'll keep scripts and other resources. Double-click on `C:/dev/fyrox_lua_sdk/fyroxed_lua.exe` and choose that directory.

Game can be launched using Play button in editor. Also, game can be launched using `C:/dev/fyrox_lua_sdk/fyrox_lite_lua.exe c:/dev/my_lua_game` command.

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

Inherit classes from `Script` to create scripts that can be attached to scene nodes in editor via dropdown on Inspector panel at the right. for `@uuid` annotation value [can be generated here](https://www.uuidgenerator.net/).

Inherit from `GlobalScript` to create singleton scripts. Unlike C# scripts, Lua global scripts require UUID.

For details about `Fyrox Lite API`, look into [guards_lua](showcase/guards_lua) project as a reference.

## Scripts Reloading
Editor reloads scripts metadata automatically after their change and window re-focusing. In-game hot reload works the same way.

## Scenes & Assets
* [Fyrox Book: Editor Overview](https://fyrox-book.github.io/beginning/editor_overview.html)
* [Fyrox Book: Assets](https://fyrox-book.github.io/beginning/assets.html)

## CLI
The `fyroxed_lua.exe` can also be invoked via terminal with project path argument.
