use proc_macro::TokenStream;

#[proc_macro_derive(Partial, attributes(partially))]
pub fn derive_partial(input: TokenStream) -> TokenStream {
    todo!()
}
