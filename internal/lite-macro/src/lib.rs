use proc_macro::TokenStream;

extern crate proc_macro;

/// placed on the `impl` of the struct or impl.
/// * it implicitly requires all exported methods to be in a single `impl` block.
/// * all methods with receiver will be registered as instance methods, all methods without - as class (aka static) methods.
/// * it's up to language provider to expose `get_` and `set_` instance methods as a fields or properties, if language allows it.
/// * example: `#[class_impl(MyClass)]`
#[proc_macro_attribute]
pub fn lite_api(attr: TokenStream, item: TokenStream) -> TokenStream {
    lite_macro_lib::fyrox_lite::lite_api(attr.into(), item.into()).into()
}
