use internal::expand_derive_partial;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod internal;

#[proc_macro_derive(Partial, attributes(partially))]
pub fn derive_partial(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);

    expand_derive_partial(&mut input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_attribute]
pub fn __derive(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
