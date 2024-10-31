use darling::{
    ast,
    util::{Flag, PathList},
    FromDeriveInput,
};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_quote, Attribute, Generics, Ident, Path, Visibility};

use super::{
    field_receiver::FieldReceiver,
    impl_partial::ImplPartial,
    meta_attribute::MetaAttribute,
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
    ///
    /// Note: This is technically just a specific sub-case of [`Self::additional_attrs`]
    /// but exists on it's own as a cleaner shortcut.
    pub derive: Option<PathList>,

    /// Recieves a [`Vec<Meta>`] containing entries to
    /// prepend as attributes to the generated struct.
    ///
    /// For example: `#[partially(attribute(serde(default))]` would result in `#[serde(default)]`
    /// being appended to the generated struct.
    #[darling(rename = "attribute", multiple)]
    pub additional_attrs: Vec<MetaAttribute>,

    /// Recieves an optional flag that indicates we should not forward attributes
    /// from the original struct to the generated struct.
    ///
    /// Note: By default, `false` - meaning __we will forward attributes__.
    ///
    /// Note: If [`Self::additional_attrs`] is present, those attributes __will still be added__
    /// to the generated struct.
    ///
    /// Note: If [`Self::derive`] is present, that attributes __will still be added__ to
    /// the generated struct.
    #[darling(rename = "skip_attributes")]
    pub skip_attrs: Flag,

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
            ref additional_attrs,
            ref skip_attrs,
            ref krate,
        } = *self;

        // parse the crate config, or use `partially` for the crate path
        let krate = if let Some(krate) = krate {
            krate.to_owned()
        } else {
            parse_quote!(partially)
        };

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
        if !skip_attrs.is_present() {
            for attr in attrs {
                if !attr.path().is_ident("derive") {
                    tokens.extend(quote!(#attr))
                }
            }
        }

        // handle additional attrs
        let additional_attrs =
            TokenVec::new_with_vec_and_sep(additional_attrs.to_vec(), Separator::Newline);
        tokens.extend(quote! {
            #additional_attrs
        });

        let mut field_tokens = TokenStream::new();
        for field in &fields {
            field.to_tokens(&mut field_tokens, &krate);
            field_tokens.extend(quote!(,));
        }

        // write the struct
        tokens.extend(quote! {
            #vis struct #to_ident #ty #wher {
                #field_tokens
            }
        });

        // create the impl
        let impl_partial = ImplPartial {
            krate: &krate,
            from_ident: ident,
            to_ident: &to_ident,
            generics,
            fields: &fields,
        };

        // write the impl
        tokens.extend(quote! {
            #impl_partial
        });

        // create the partial => partial impl
        let partial_impl_partial = ImplPartial {
            krate: &krate,
            from_ident: &to_ident,
            to_ident: &to_ident,
            generics,
            fields: &fields,
        };

        // write it
        tokens.extend(quote! {
            #partial_impl_partial
        })
    }
}
