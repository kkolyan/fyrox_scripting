- [Lite API](#lite-api)
- [Contract](#contract)
- [Language Implementations](#language-implementations)
- [Lua Implementation](#lua-implementation)

### Project Structure

* [api](api): crates that defines the facade for scripting.
    * [fyrox-lite](api/fyrox-lite): Fyrox Lite - scripting-language-friendly facade over the Fyrox API. Automatically
      exposed to all scripting languages. Supposed to be the most frequently changing part of this project.
    * [fyrox-lite-color](api/fyrox-lite-color), [fyrox-lite-math](api/fyrox-lite-math): color and math related API for
      dynamic scripting languages. Static scripting languages supposed to use their own implementations.
* [bash](bash): bash scripts to automate different build operations. On Windows it's supposed to use Git Bash (I haven't
  tried another MSYS2 distributions in this project).
* [engine](engine): Fyrox engine as Git submodule. Probably, in future it will be removed in favor to referencing toi
  fyrox as a crate,
  but for now it's very useful to have its code at hand.
* [internal](internal): generic internal things.
* [langs](langs): scripting language specific internal things.
* [showcase](showcase): projects with useful examples.
* [tools](tools): programs for bindings generation.
    * [csgen.rs](tools/src/bin/csgen.rs): update C# adapter. should be run after changes
      in [fyrox-lite](api/fyrox-lite).
    * [luagen.rs](tools/src/bin/luagen.rs): update Lua adapter. should be run after changes
      in [fyrox-lite](api/fyrox-lite), [fyrox-lite-color](api/fyrox-lite-color)
      or [fyrox-lite-math](api/fyrox-lite-math).
    * [mdgen.rs](tools/src/bin/mdgen.rs): update mdbook. should be run after changes
      in [fyrox-lite](api/fyrox-lite), [fyrox-lite-color](api/fyrox-lite-color)
      or [fyrox-lite-math](api/fyrox-lite-math).
    * [allgen.rs](tools/src/bin/allgen.rs): updates all. use it if in doubt.
* [www](www): sources for [the site](https://kkolyan.github.io/fyrox_lite/). It consists from
  3 [mdbook](https://rust-lang.github.io/mdBook/)-driven sites, glued together. It's published automatically after
  changes of md files using GitHub Actions. `scripting_api` directories in both language-specific sites are created and
  updated
  using [mdgen.rs](tools/src/bin/mdgen.rs).
    * [hub](www/hub): language-agnostic part (including main page)
    * [fyrox_cs](www/fyrox_cs): C#-specific part
    * [fyrox_lua](www/fyrox_lua): Lua-specific part

### Lite API

Lite API is a Rust library that provides a scripting-language-friendly facade over the Fyrox API. It isn't bound to a
specific language, but its design assumes that scripting language has GC and some kind of OOP.

This library is supposed to be updated frequently when it's necessary to expose some part of Fyrox API to scripting
language. Package [fyrox-lite](api/fyrox-lite) is the place where most of the changes to be done.

Exposed API should comply with the rules. Following types allowed (owned only, no references allowed):

* primitives (limited set of them, for the sake of simplicity)
* `data types` - `#[fyrox_lite]`-annotated structures or enums. they have copy-on-asign semantic. It's supposed that on
  the scripting language side they are represented in its native data structures. That's not allowed to expose Rust
  methods of this structures - all necessary methods should be provided by the language specific implementation.
* `engine types` - defined by annotating non-trait `impl`s with this same `#[fyrox_lite]` attribute. Script code can
  invoke exposed methods (using `ffi` or analogs), but internal structure of this types is completely hidden. Script
  code can instantiate an engine type only if there is exposed method for this. Handles are clonable and clone operation
  only clones the handle, not the underlying object. If underlying object has limited lifecycle, then it should provide
  the methods to deal with it.
* predefined abstract types. That's a family of traits, expected to be implemented by every language provider. they are
  not intended to be changed frequently. The central type is [UserScript](api/fyrox-lite/src/spi.rs).
* `Vec<T>`, `Option<T>`, `Result<T>` where `T` is allowed type..

Note that Vector3 and Quaternion for Lua are of an `engine type`, but for some languages (C# for instance) they probably
would be a `data type`, because language-native implementation of vector arithmetics could be more efficient than `ffi`
to `nalgebra`. That's why nalgebra-backed types are in [fyrox-lite-math](api/fyrox-lite-math)
and [fyrox-lite](api/fyrox-lite) exposes methods with shallow math structs instead of nalgebra-backed ones.

`#[fyrox_lite]` attrubute is not just a marker - it provides almost complete realtime enforcement of these rules.

### Contract

There is a [metadata model](internal/lite-model/src/lib.rs) that serves as contract between `Lite API` and
`Language Implementation`s. There is the package `lite-parser` that is responsible for collecting metadata using this
same `#[lite_api]` attribute. For the debug purposes, collected metadata is dumped in
json ([fyrox-lite](api/fyrox-lite/src/domain.json), [fyrox-lite-math](api/fyrox-lite-math/src/domain.json)).

### Language Implementations

There is no specific rules for this, but it's supposed that language implementation consumes the Lite API metadata and
produces a Rust code with Fyrox `Plugin` implementation that loads scripts metadata (script names, property types and
names), allowing attaching them in inspector, and provides a runtime for a target scripting language.

### Lua Implementation

* `langs/lua/fyrox-lite-lua-lib` - the runtime library,
  provides [LuaPlugin](langs/lua/fyrox-lite-lua-lib/src/fyrox_lua_plugin.rs)
  and [ExternalScriptProxy](langs/lua/fyrox-lite-lua-lib/src/external_script_proxy.rs). [mlua](https://github.com/mlua-rs/mlua)
  crate used to embed Lua. LuaU interpreter is choosen (mlua allow to switch them easily) just because it was easiest to
  compile on Windows, but there is no dependency on specific interpreter features.
* `langs/lua/fyroxed-lua` / `langs/lua/fyrox-lite-lua` - desktop instantiations of previously mentioned `LuaPlugin`.
* `langs/lua/luagen-lib` - dynamic part. It uses Lite API metadata to generate
  both [Lua bindings](langs/lua/fyrox-lite-lua-lib/src/generated) and [Lua annotations](langs/lua/annotations).
  Currently, `luagen-lib` is not integrated with build and invoked with
  `cargo run --bin luagen` ([code](tools/src/bin/luagen.rs)).