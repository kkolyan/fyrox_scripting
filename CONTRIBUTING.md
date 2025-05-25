
- [Lite API](#lite-api)
- [Contract](#contract)
- [Language Implementations](#language-implementations)
- [Lua Implementation](#lua-implementation)

### Lite API
Lite API is a Rust library that provides a scripting-language-friendly facade over the Fyrox API. It isn't bound to a specific language, but it's design assumes that scripting language has GC and some kind of OOP.

This library is supposed to be updated frequently when it's necessary to expose some part of Fyrox API to scripting language. Package [fyrox-lite](fyrox-lite) is the place where most of the changes to be done.

Exposed API should comply with the rules. Following types allowed (owned only, no references allowed):
* primitives (limited set of them, for the sake of simplicity)
* `data types` - `#[fyrox_lite]`-annotated structures or enums. they have copy-on-asign semantic. It's supposed that on the scripting language side they are represented in its native data structures. That's not allowed to expose Rust methods of this structures - all necessary methods should be provided by the language specific implementation.
* `engine types` - defined by annotating non-trait `impl`s with this same `#[fyrox_lite]` attribute. Script code can invoke exposed methods (using `ffi` or analogs), but internal structure of this types is completely hidden. Script code can instantiate an engine type only if there is exposed method for this. Handles are clonable and clone operation only clones the handle, not the underlying object. If underlying object has limited lifecycle, then it should provide the methods to deal with it.
* predefined abstract types. That's a family of traits, expected to be implemented by every language provider. they are not intended to be changed frequently. The central type is [UserScript](fyrox-lite/src/spi.rs).
* `Vec<T>`, `Option<T>`, `Result<T>` where `T` is allowed type..

Note that Vector3 and Quaternion for Lua are of an `engine type`, but for some languages (C# for instance) they probably would be a `data type`, because language-native implementation of vector arithmetics could be more efficient than `ffi` to `nalgebra`. That's why nalgebra-backed types are in [fyrox-lite-math](fyrox-lite-math) and [fyrox-lite](fyrox-lite) exposes methods with shallow math structs instead of nalgebra-backed ones.

`#[fyrox_lite]` attrubute is not just a marker - it provides almost complete realtime enforcement of this rules.

### Contract
There is a [metadata model](lite-model/src/lib.rs) that serves as contract between `Lite API` and `Language Implementation`s. There is the package `lite-parser` that is responsible for collecting metadata using this same `#[lite_api]` attribute. For the debug purposes, collected metadata is dumped in json ([fyrox-lite](fyrox-lite/src/domain.json), [fyrox-lite-math](fyrox-lite-math/src/domain.json)).

### Language Implementations
There is no specific rules for this, but it's supposed that language implementation consumes the Lite API metadata and produces a Rust code with Fyrox `Plugin` implementation that loads scripts metadata (script names, property types and names), allowing attaching them in inspector, and provides a runtime for a target scripting language.

### Lua Implementation
* `langs/lua/fyrox-lite-lua-lib` - the runtime library, provides [LuaPlugin](langs/lua/fyrox-lite-lua-lib/src/fyrox_lua_plugin.rs) and [ExternalScriptProxy](langs/lua/fyrox-lite-lua-lib/src/external_script_proxy.rs). [mlua](https://github.com/mlua-rs/mlua) crate used to embed Lua. LuaU interpreter is choosen (mlua allow to switch them easily) just because it was easiest to compile on Windows, but there is no dependency on specific interpreter features.
* `langs/lua/fyroxed-lua` / `langs/lua/fyrox-lite-lua` - desktop instantiations of previously mentioned `LuaPlugin`.
* `langs/lua/luagen-lib` - dynamic part. It uses Lite API metadata to generate both [Lua bindings](langs/lua/fyrox-lite-lua-lib/src/generated) and [Lua annotations](langs/lua/annotations). Currently, `luagen-lib` is not integrated with build and invoked with `cargo run --bin luagen` ([code](tools/src/bin/luagen.rs)).