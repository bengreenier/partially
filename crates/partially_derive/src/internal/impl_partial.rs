use quote::{quote, ToTokens};
use syn::{parse_quote, Generics, Ident, Path};

use super::{
    field_receiver::FieldReceiver,
    token_vec::{Separator, TokenVec},
};

pub struct ImplPartial<'a> {
    pub krate: &'a Option<Path>,
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

        // parse the crate config, or use `partially` for the crate path
        let krate = if let Some(krate) = krate {
            krate.to_owned()
        } else {
            parse_quote!(partially)
        };

        let field_applicators = fields
            .iter()
            .map(|f| {
                // // this is enforced with a better error by [`FieldReceiver::validate`].
                let from_ident = f.ident.as_ref().unwrap();

                let to_ident = f.rename.as_ref().unwrap_or(from_ident);

                quote! {
                    if let Some(#to_ident) = partial.#to_ident {
                        self.#from_ident = #to_ident.into();
                    }
                }
            })
            .collect();
        let field_applicators =
            TokenVec::new_with_vec_and_sep(field_applicators, Separator::Newline);

        tokens.extend(quote! {
            impl #imp #krate::Partial for #from_ident #ty #wher {
                type Item = #to_ident #ty;

                fn apply_some(&mut self, partial: Self::Item) {
                    #field_applicators
                }
            }
        })
    }
}
