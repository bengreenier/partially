use darling::{ast, util::PathList, FromDeriveInput};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{Attribute, Generics, Ident, Path, Visibility};

use super::{
    field_receiver::FieldReceiver,
    impl_partial::ImplPartial,
    token_vec::{Separator, TokenVec},
};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(partially), supports(struct_any), forward_attrs)]
pub struct DeriveReceiver {
    /// The struct ident.
    pub ident: Ident,

    /// Get the visibility of the field.
    pub vis: Visibility,

    /// Get the attributes of the field.
    pub attrs: Vec<Attribute>,

    /// The type's generics. You'll need these any time your trait is expected
    /// to work with types that declare generics.
    pub generics: Generics,

    /// Receives the body of the struct. We don't care about
    /// struct fields because we previously told darling we only accept structs.
    pub data: ast::Data<(), FieldReceiver>,

    /// Receives an optional identifer to use for the generated struct.
    ///
    /// Note: By default, `Partial` + [`Self::ident`] is used.  
    pub rename: Option<Ident>,

    /// Receives an optional [`PathList`] defining the various
    /// derive entries for the generated struct to `#[derive()]`
    ///
    /// Note: By default, [`None`].
    pub derive: Option<PathList>,

    /// Receives an optional [`Path`] defining the path to the `partially` crate.
    #[darling(rename = "crate")]
    pub krate: Option<Path>,
}

impl ToTokens for DeriveReceiver {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let DeriveReceiver {
            ref ident,
            ref vis,
            ref attrs,
            ref generics,
            ref data,
            ref rename,
            ref derive,
            ref krate,
        } = *self;

        let (_, ty, wher) = generics.split_for_impl();

        let fields: Vec<_> = data
            .as_ref()
            .take_struct()
            .expect("expected a struct")
            .fields
            .into_iter()
            .filter(|f| !f.omit.is_present())
            .collect();

        let to_ident = if let Some(rename) = &rename {
            rename.to_owned()
        } else {
            let title = format!("Partial{}", ident);
            Ident::new(&title, ident.span())
        };

        // handle custom derive attr
        if let Some(derive_paths) = derive {
            let derive_paths =
                TokenVec::new_with_vec_and_sep(derive_paths.to_vec(), Separator::Comma);
            tokens.extend(quote! {
                    #[derive(#derive_paths)]
            });
        }

        // handle non-derive attrs
        for attr in attrs {
            if !attr.path().is_ident("derive") {
                tokens.extend(quote!(#attr))
            }
        }

        let field_tokens = TokenVec::new_with_vec_and_sep(fields.clone(), Separator::CommaNewline);

        // write the struct
        tokens.extend(quote! {
            #vis struct #to_ident #ty #wher {
                #field_tokens
            }
        });

        // create the impl
        let impl_partial = ImplPartial {
            krate,
            from_ident: ident,
            to_ident: &to_ident,
            generics,
            fields: &fields,
        };

        // write the impl
        tokens.extend(quote! {
            #impl_partial
        })
    }
}
