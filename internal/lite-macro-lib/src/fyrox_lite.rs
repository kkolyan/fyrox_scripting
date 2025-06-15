use lite_parser::{
    extract_engine_class::extract_engine_class_and_inject_assertions,
    extract_pod_enum::extract_pod_enum, extract_pod_struct::extract_pod_struct,
    lite_api_attr::LiteApiAttr,
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, spanned::Spanned};

use crate::generate_static_assertions;

pub fn lite_api(attr: TokenStream, item: TokenStream) -> TokenStream {
    let span = attr.span();
    let mut errors = Vec::new();
    let attr = match LiteApiAttr::from_attr_args(attr) {
        Ok(it) => it,
        Err(err) => return err.to_compile_error(),
    };
    match parse2::<syn::Item>(item) {
        Ok(it) => match it {
            syn::Item::Enum(item) => {
                let mut types = Vec::new();
                let ident =
                    extract_pod_enum("dontcare", (attr, span), &item, &mut errors, &mut types)
                        .map(|(rust_class_name, _class)| rust_class_name);

                let field_assertions = generate_static_assertions(types.iter());

                let errors = errors
                    .into_iter()
                    .map(|it| it.into_compile_error())
                    .collect::<TokenStream>();

                let impl_lite_data_type =
                    ident.map(|ident| quote! {impl crate::LiteDataType for #ident {}});

                quote! {
                    #errors
                    // #[derive(Debug, Clone)]
                    #item

                    #impl_lite_data_type
                    #field_assertions
                }
            }
            syn::Item::Struct(item) => {
                extract_pod_struct("dontcare", (attr, span), &item, &mut errors);

                let ident = &item.ident;

                let errors = errors
                    .into_iter()
                    .map(|it| it.into_compile_error())
                    .collect::<TokenStream>();

                let field_assertions =
                    generate_static_assertions(item.fields.iter().map(|it| &it.ty));
                quote! {
                    #errors
                    // #[derive(Debug, Clone)]
                    #item

                    impl crate::LiteDataType for #ident {}
                    #field_assertions

                }
            }
            syn::Item::Impl(mut item) => {
                let ident = extract_engine_class_and_inject_assertions(
                    "dontcare",
                    (attr, span),
                    &mut item,
                    &mut errors,
                )
                .map(|(rust_class_name, _class)| rust_class_name);

                let errors = errors
                    .into_iter()
                    .map(|it| it.into_compile_error())
                    .collect::<TokenStream>();

                let impl_lite_data_type =
                    ident.map(|ident| quote! {impl crate::LiteDataType for #ident {}});

                quote! {
                    #errors
                    #item

                    // impl_lite_data_type
                    #impl_lite_data_type
                }
            }
            it => {
                let error = syn::Error::new(
                    span,
                    "fyrox_lite allowed only for impl, struct or enum declarations",
                )
                .into_compile_error();
                quote! {
                    #error
                    #it
                }
            }
        },
        Err(err) => err.to_compile_error(),
    }
}
