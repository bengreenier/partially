use quote::{quote, ToTokens};
use syn::{Generics, Ident};

use super::{
    field_receiver::FieldReceiver,
    token_vec::{Separator, TokenVec},
};

pub struct ImplFromFull<'a> {
    pub generics: &'a Generics,
    pub full_ident: &'a Ident,
    pub partial_ident: &'a Ident,
    pub fields: &'a Vec<&'a FieldReceiver>,
}

impl<'a> ToTokens for ImplFromFull<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self {
            generics,
            full_ident,
            partial_ident,
            fields,
        } = self;

        let (imp, ty, wher) = generics.split_for_impl();

        let field_inits = fields
            .iter()
            .map(|f| {
                let from_ident = f.ident.as_ref().unwrap();
                let to_ident = f.rename.as_ref().unwrap_or(from_ident);

                if f.transparent.is_present() {
                    quote! {
                        #to_ident: full.#from_ident
                    }
                } else {
                    quote! {
                        #to_ident: Some(full.#from_ident.into())
                    }
                }
            })
            .collect::<Vec<_>>();
        let field_inits = TokenVec::new_with_vec_and_sep(field_inits, Separator::CommaNewline);

        tokens.extend(quote! {
            impl #imp core::convert::From<#full_ident #ty> for #partial_ident #ty #wher {
                fn from(full: #full_ident #ty) -> Self {
                    Self {
                        #field_inits
                    }
                }
            }
        });
    }
}
