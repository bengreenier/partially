use darling::{util::Flag, FromField, Result};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, Ident, Path, Type, Visibility};

#[derive(Debug, FromField)]
#[darling(attributes(partially), forward_attrs, and_then = FieldReceiver::validate)]
pub struct FieldReceiver {
    /// Get the ident of the field. For fields in tuple or newtype structs or
    /// enum bodies, this can be [`None`].
    pub ident: Option<Ident>,

    /// Get the visibility of the field.
    pub vis: Visibility,

    /// Get the attributes of the field.
    pub attrs: Vec<syn::Attribute>,

    /// This magic field name pulls the type from the input.
    pub ty: Type,

    /// An optional identifer to use for the generated field.
    ///
    /// Note: By default, `Partial` + [`Self::ident`] is used.  
    pub rename: Option<Ident>,

    /// A flag indicating that the given field should be omitted from generation.
    ///
    /// Note: This will create a generated struct that is missing the [`Self::ident`] field.
    pub omit: Flag,

    /// A flag indicating that the given field should not be "partial-ized" and instead
    /// should be directly forwarded to the child.
    ///
    /// Note: This means that [`Self::ty`] will be used for the generated field, rather than [`Option<Self::ty>`].
    pub transparent: Flag,

    /// An optional type override to use for the generated field.
    ///
    /// Note: If specified, the given [`Type`] will be used verbatim, not wrapped in an [`Option`].
    /// Note: By default, [`Option<Self::ty>`] is used.
    pub as_type: Option<Type>,

    /// An optional flag that indicates that this field has a type that is [`Partial`] and should
    /// be treated as such.
    pub nested: Flag,
}

impl FieldReceiver {
    fn validate(self) -> Result<Self> {
        let mut acc = darling::Error::accumulator();

        if self.ident.is_none() {
            acc.push(darling::Error::custom(
                "cannot use rename on an unnamed field",
            ))
        }

        if self.omit.is_present()
            && (self.rename.is_some()
                || self.transparent.is_present()
                || self.as_type.is_some()
                || self.nested.is_present())
        {
            acc.push(darling::Error::custom(
                "cannot use omit with any other options",
            ));
        }

        if self.transparent.is_present() as i32
            + self.as_type.is_some() as i32
            + self.nested.is_present() as i32
            > 1
        {
            acc.push(darling::Error::custom(
                "transparent, as_type and nested are mutually exclusive",
            ));
        }

        acc.finish_with(self)
    }

    pub fn to_tokens(&self, tokens: &mut TokenStream, krate: &Path) {
        if self.omit.is_present() {
            return;
        }

        // this is enforced with a better error by [`FieldReceiver::validate`].
        let src_name = self.ident.as_ref().expect("expected a named field");

        let dst_name = if let Some(name) = &self.rename {
            name
        } else {
            src_name
        };

        let src_type = &self.ty;
        let dst_type = if self.transparent.is_present() {
            src_type.to_owned()
        } else if let Some(ty) = &self.as_type {
            ty.to_owned()
        } else if self.nested.is_present() {
            parse_quote! {
                <#src_type as #krate::Partial>::Item
            }
        } else {
            let ty: Type = parse_quote! {
                Option<#src_type>
            };

            ty
        };

        let vis = &self.vis;
        let forwarded_attrs = &self.attrs;

        for attr in forwarded_attrs {
            tokens.extend(quote! {
                #attr
            })
        }

        tokens.extend(quote! {
            #vis #dst_name: #dst_type
        })
    }
}

#[cfg(test)]
mod test {
    use darling::util::Flag;
    use proc_macro2::Span;
    use quote::quote;
    use syn::Ident;

    use super::FieldReceiver;

    fn make_dummy() -> FieldReceiver {
        FieldReceiver {
            ident: Some(Ident::new("Dummy", Span::call_site())),
            vis: syn::Visibility::Public(syn::token::Pub::default()),
            attrs: Vec::new(),
            ty: syn::Type::Verbatim(quote!(DummyField)),
            rename: None,
            omit: Flag::default(),
            transparent: Flag::default(),
            as_type: None,
            nested: Flag::default(),
        }
    }

    #[test]
    fn invalidates_no_ident() {
        let mut instance = make_dummy();
        instance.ident = None;

        if let Ok(e) = instance.validate() {
            println!("{:?}", e)
        }
    }

    #[test]
    fn invalidate_omit_rename() {
        let mut instance = make_dummy();
        instance.omit = Flag::present();
        instance.rename = Some(Ident::new("Renamed", Span::call_site()));

        assert!(instance.validate().is_err())
    }

    #[test]
    fn invalidate_omit_transparent() {
        let mut instance = make_dummy();
        instance.omit = Flag::present();
        instance.transparent = Flag::present();

        assert!(instance.validate().is_err())
    }

    #[test]
    fn invalidate_omit_as_type() {
        let mut instance = make_dummy();
        instance.omit = Flag::present();
        instance.as_type = Some(syn::Type::Verbatim(quote!(NewDummyField)));

        assert!(instance.validate().is_err())
    }

    #[test]
    fn invalidate_omit_nested() {
        let mut instance = make_dummy();
        instance.omit = Flag::present();
        instance.nested = Flag::present();

        assert!(instance.validate().is_err())
    }

    #[test]
    fn invalidate_transparent_as_type_nested() {
        // as_type + transparent
        let mut instance = make_dummy();
        instance.as_type = Some(syn::Type::Verbatim(quote!(NewDummyField)));
        instance.transparent = Flag::present();
        assert!(instance.validate().is_err());

        // as_type + transparent + nested
        let mut instance = make_dummy();
        instance.as_type = Some(syn::Type::Verbatim(quote!(NewDummyField)));
        instance.transparent = Flag::present();
        instance.nested = Flag::present();
        assert!(instance.validate().is_err());

        // as_type + nested
        let mut instance = make_dummy();
        instance.as_type = Some(syn::Type::Verbatim(quote!(NewDummyField)));
        instance.nested = Flag::present();
        assert!(instance.validate().is_err());

        // nested + transparent
        let mut instance = make_dummy();
        instance.transparent = Flag::present();
        instance.nested = Flag::present();
        assert!(instance.validate().is_err());
    }


    #[test]
    fn validate() {
        let instance = make_dummy();

        assert!(instance.validate().is_ok());
    }
}
