use internal::expand_derive_partial;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod internal;

/// The `derive` macro for `partially_derive::Partial`.
#[proc_macro_derive(Partial, attributes(partially))]
pub fn derive_partial(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);

    expand_derive_partial(&mut input).into()
}
