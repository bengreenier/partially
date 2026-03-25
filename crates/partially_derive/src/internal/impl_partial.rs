use quote::{quote, ToTokens};
use syn::{Generics, Ident, Path};

use super::{
    field_receiver::FieldReceiver,
    token_vec::{Separator, TokenVec},
};

pub struct ImplPartial<'a> {
    pub krate: &'a Path,
    pub generics: &'a Generics,
    pub from_ident: &'a Ident,
    pub to_ident: &'a Ident,

    /// Note: assumed to already be filtered (such that `omit`-ted entries are removed)
    pub fields: &'a Vec<&'a FieldReceiver>,
}

impl<'a> ToTokens for ImplPartial<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self {
            krate,
            from_ident,
            to_ident,
            generics,
            fields,
        } = self;

        let (imp, ty, wher) = generics.split_for_impl();

        let has_nested = fields.iter().any(|f| f.nested.is_present());

        let non_nested_is_somes: Vec<_> = fields
            .iter()
            .filter(|f| !f.nested.is_present())
            .map(|f| {
                let from_ident = f.ident.as_ref().unwrap();
                let to_ident = f.rename.as_ref().unwrap_or(from_ident);
                quote!(partial.#to_ident.is_some())
            })
            .collect();

        let will_apply_some_binding = if has_nested {
            if non_nested_is_somes.is_empty() {
                quote!(let mut will_apply_some = false;)
            } else {
                let is_somes =
                    TokenVec::new_with_vec_and_sep(non_nested_is_somes, Separator::Or);
                quote!(let mut will_apply_some = #is_somes;)
            }
        } else {
            let is_somes =
                TokenVec::new_with_vec_and_sep(non_nested_is_somes, Separator::Or);
            quote!(let will_apply_some = #is_somes;)
        };

        let field_applicators: Vec<_> = fields
            .iter()
            .map(|f| {
                let from_ident = f.ident.as_ref().unwrap();
                let to_ident = f.rename.as_ref().unwrap_or(from_ident);

                if f.nested.is_present() {
                    quote! {
                        will_apply_some = #krate::Partial::apply_some(
                            &mut self.#from_ident,
                            partial.#to_ident
                        ) || will_apply_some;
                    }
                } else {
                    quote! {
                        if let Some(#to_ident) = partial.#to_ident {
                            self.#from_ident = #to_ident.into();
                        }
                    }
                }
            })
            .collect();
        let field_applicators =
            TokenVec::new_with_vec_and_sep(field_applicators, Separator::Newline);

        tokens.extend(quote! {
            impl #imp #krate::Partial for #from_ident #ty #wher {
                type Item = #to_ident #ty;

                fn apply_some(&mut self, partial: Self::Item) -> bool {
                    #will_apply_some_binding

                    #field_applicators

                    will_apply_some
                }
            }
        })
    }
}
