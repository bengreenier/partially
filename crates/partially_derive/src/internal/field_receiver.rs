use darling::{util::Flag, FromField, Result};
use quote::{quote, ToTokens};
use syn::{parse_quote, Ident, PathArguments, Type, Visibility};

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
}

impl FieldReceiver {
    fn auto_nested_ty(&self) -> Option<Type> {
        if self.omit.is_present() || self.transparent.is_present() || self.as_type.is_some() {
            return None;
        }

        let Type::Path(mut path) = self.ty.clone() else {
            return None;
        };

        if path.qself.is_some() {
            return None;
        }

        let first_ident = path.path.segments.first().map(|s| s.ident.to_string())?;
        if matches!(first_ident.as_str(), "std" | "core" | "alloc") {
            return None;
        }

        let last = path.path.segments.last_mut()?;
        match last.arguments {
            PathArguments::None | PathArguments::AngleBracketed(_) => {}
            _ => return None,
        }

        let type_name = last.ident.to_string();
        if !type_name
            .chars()
            .next()
            .map(|ch| ch.is_ascii_uppercase())
            .unwrap_or(false)
        {
            return None;
        }

        if type_name.starts_with("Partial")
            || matches!(
                type_name.as_str(),
                "String"
                    | "Self"
                    | "bool"
                    | "char"
                    | "str"
                    | "i8"
                    | "i16"
                    | "i32"
                    | "i64"
                    | "i128"
                    | "isize"
                    | "u8"
                    | "u16"
                    | "u32"
                    | "u64"
                    | "u128"
                    | "usize"
                    | "f32"
                    | "f64"
                    | "Option"
                    | "Vec"
                    | "Box"
                    | "Rc"
                    | "Arc"
                    | "Cow"
                    | "Result"
                    | "HashMap"
                    | "BTreeMap"
                    | "HashSet"
                    | "BTreeSet"
            )
            || (type_name.len() == 1
                && type_name
                    .chars()
                    .next()
                    .map(|ch| ch.is_ascii_uppercase())
                    .unwrap_or(false))
        {
            return None;
        }

        let partial_name = format!("Partial{type_name}");
        last.ident = Ident::new(&partial_name, last.ident.span());

        Some(Type::Path(path))
    }

    pub fn is_auto_nested(&self) -> bool {
        self.auto_nested_ty().is_some()
    }

    fn validate(self) -> Result<Self> {
        let mut acc = darling::Error::accumulator();

        if self.ident.is_none() {
            acc.push(darling::Error::custom(
                "cannot use rename on an unnamed field",
            ))
        }

        if self.omit.is_present()
            && (self.rename.is_some() || self.transparent.is_present() || self.as_type.is_some())
        {
            acc.push(darling::Error::custom(
                "cannot use omit with any other options",
            ));
        }

        if self.transparent.is_present() && self.as_type.is_some() {
            acc.push(darling::Error::custom(
                "cannot use both transparent and as_type",
            ));
        }

        acc.finish_with(self)
    }
}

impl ToTokens for FieldReceiver {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
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
        } else if let Some(nested) = self.auto_nested_ty() {
            nested
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
    fn invalidate_transparent_as_type() {
        let mut instance = make_dummy();
        instance.transparent = Flag::present();
        instance.as_type = Some(syn::Type::Verbatim(quote!(NewDummyField)));

        assert!(instance.validate().is_err())
    }

    #[test]
    fn validate() {
        let instance = make_dummy();

        assert!(instance.validate().is_ok());
    }
}
