use quote::{quote, ToTokens};
use std::iter;
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

        let field_is_somes = iter::once(quote!(false))
            .chain(fields.iter().filter(|f| !f.nested.is_present()).map(|f| {
                // this is enforced with a better error by [`FieldReceiver::validate`].
                let from_ident = f.ident.as_ref().unwrap();

                let to_ident = f.rename.as_ref().unwrap_or(from_ident);

                quote!(partial.#to_ident.is_some())
            }))
            .collect();
        let field_is_somes = TokenVec::new_with_vec_and_sep(field_is_somes, Separator::Or);

        let field_applicators = fields
            .iter()
            .map(|f| {
                // this is enforced with a better error by [`FieldReceiver::validate`].
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
                    let mut will_apply_some = #field_is_somes;

                    #field_applicators

                    will_apply_some
                }
            }
        })
    }
}
