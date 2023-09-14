use std::fmt::Debug;

use proc_macro2::{Punct, Span};
use quote::ToTokens;
use syn::{parse::Parse, punctuated::Punctuated, Attribute, Ident, Lit, Meta, Result, Token, Type};

use super::{
    ctx::ContextQuery,
    symbol::{AS_TYPE, PARTIALLY, RENAME, TRANSPARENT},
};

/// The set of operations that we may be instructed to take via `#[partially()]` attributes.
#[derive(Default)]
pub struct OperationSet {
    rename: Option<Ident>,
    transparent: bool,
    as_type: Option<Type>,
}

impl OperationSet {
    /// Obtains the rename operation value, if any.
    pub fn rename(&self) -> Option<Ident> {
        self.rename.clone()
    }

    /// Obtains the transparent operation value.
    pub fn transparent(&self) -> bool {
        self.transparent
    }

    /// Obtains the as_type operation value, if any.
    pub fn as_type(&self) -> Option<Type> {
        self.as_type.clone()
    }

    /// Creates a new [`OperationSet`] by parsing a set of attributes within a context.
    pub fn new<Q: ContextQuery>(ctx: &Q, attributes: &Vec<Attribute>) -> Result<OperationSet> {
        let mut known_attributes = OperationSet::default();

        for attr in attributes {
            if attr.path() != PARTIALLY {
                continue;
            }

            if let Meta::List(meta) = &attr.meta {
                if meta.tokens.is_empty() {
                    continue;
                }
            }

            let args = attr.parse_args_with(Punctuated::<Argument, Token![,]>::parse_terminated)?;

            for arg in args {
                match arg {
                    Argument::Standalone(ident) => {
                        if ident == TRANSPARENT
                            && ctx.is_field()
                            && known_attributes.supports_transparent()
                        {
                            known_attributes.transparent = true;
                        } else {
                            return Err(syn::Error::new_spanned(ident, "unsupported argument"));
                        }
                    }
                    Argument::WithLiteral(ident, value) => {
                        if ident == RENAME && known_attributes.supports_rename() {
                            known_attributes.rename = Some(Ident::new(&value, Span::call_site()));
                        } else {
                            return Err(syn::Error::new_spanned(ident, "unsupported argument"));
                        }
                    }
                    Argument::WithType(ident, ty) => {
                        if ident == AS_TYPE && ctx.is_field() && known_attributes.supports_as_type()
                        {
                            known_attributes.as_type = Some(ty);
                        } else {
                            return Err(syn::Error::new_spanned(ident, "unsupported argument"));
                        }
                    }
                }
            }
        }

        Ok(known_attributes)
    }

    /// Internal check to determine if a rename operation can be added.
    fn supports_rename(&self) -> bool {
        // the caller can always rename
        true
    }

    /// Internal check to determine if a transparent operation can be added.
    fn supports_transparent(&self) -> bool {
        // the caller can only mark as transparent when as_type isn't present
        self.as_type.is_none()
    }

    /// Internal check to determine if an as_type operation can be added.
    fn supports_as_type(&self) -> bool {
        // the caller can only as_type when transparent isn't set
        !self.transparent
    }
}

/// An argument that may define some operation within an [`OperationSet`].
enum Argument {
    Standalone(Ident),
    WithType(Ident, Type),
    WithLiteral(Ident, String),
}

impl Parse for Argument {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let ident = input.parse::<Ident>()?;
        let lookahead = input.lookahead1();

        if lookahead.peek(Token![=]) {
            let _punct = input.parse::<Punct>()?;

            let maybe_type = input.parse::<Type>();
            let maybe_literal = input.parse::<Lit>();

            if let Ok(ty) = maybe_type {
                Ok(Self::WithType(ident, ty))
            } else if let Ok(lit) = maybe_literal {
                let lit = match &lit {
                    Lit::Str(str) => Some(str.value()),
                    Lit::ByteStr(str) => Some(String::from_utf8(str.value()).map_err(|e| {
                        syn::Error::new_spanned(
                            str,
                            format_args!("invalid literal, could not decode as utf-8: {}", e),
                        )
                    })?),
                    Lit::Byte(char) => {
                        Some(String::from_utf8(vec![char.value()]).map_err(|e| {
                            syn::Error::new_spanned(
                                char,
                                format_args!("invalid literal, could not decode as utf-8: {}", e),
                            )
                        })?)
                    }
                    Lit::Char(char) => Some(char.value().to_string()),
                    _ => None,
                };

                if let Some(lit) = lit {
                    Ok(Self::WithLiteral(ident, lit))
                } else {
                    Err(lookahead.error())
                }
            } else {
                Err(lookahead.error())
            }
        } else {
            Ok(Self::Standalone(ident))
        }
    }
}

impl Debug for Argument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Argument::Standalone(ident) => f.write_fmt(format_args!("Standalone({})", ident)),
            Argument::WithLiteral(ident, lit) => {
                f.write_fmt(format_args!("WithLiteral({} = {})", ident, lit))
            }
            Argument::WithType(ident, ty) => f.write_fmt(format_args!(
                "WithType({} = {:?})",
                ident,
                ty.to_token_stream()
            )),
        }
    }
}

#[cfg(test)]
mod test {
    use proc_macro2::Span;
    use quote::ToTokens;
    use syn::{parse_quote, punctuated::Punctuated, Field, Ident, ItemStruct, Token, Type};

    use crate::internal::ctx::{Context, ContextQuery};

    use super::{Argument, OperationSet};

    #[test]
    fn parse_struct_attributes() {
        let item: ItemStruct = parse_quote! {
            #[attr(value_one = "one", value_two, value_three = f32, value_four = four_real)]
            struct MyItem {
                field: i32
            }
        };

        assert_eq!(item.attrs.len(), 1);
        let attr = item.attrs.first().unwrap();

        let args = attr
            .parse_args_with(Punctuated::<Argument, Token![,]>::parse_terminated)
            .expect("failed to parse");
        let args: Vec<Argument> = args.into_iter().collect();

        let expected = vec![
            Argument::WithLiteral(
                Ident::new("value_one", Span::call_site()),
                "one".to_string(),
            ),
            Argument::Standalone(Ident::new("value_two", Span::call_site())),
            Argument::WithType(
                Ident::new("value_three", Span::call_site()),
                parse_quote! {
                    f32
                },
            ),
            Argument::WithType(
                Ident::new("value_four", Span::call_site()),
                parse_quote! {
                    four_real
                },
            ),
        ];

        assert_eq!(format!("{:?}", args), format!("{:?}", expected));
    }

    #[test]
    fn parse_field_attributes() {
        let item: ItemStruct = parse_quote! {
            struct MyItem {
                #[attr(value_one = "one", value_two, value_three = f32, value_four = four_real)]
                field: i32
            }
        };
        let fields: Vec<Field> = item.fields.into_iter().collect();

        assert_eq!(fields.len(), 1);
        let field = fields.first().unwrap();

        assert_eq!(field.attrs.len(), 1);
        let attr = field.attrs.first().unwrap();

        let args = attr
            .parse_args_with(Punctuated::<Argument, Token![,]>::parse_terminated)
            .expect("failed to parse");
        let args: Vec<Argument> = args.into_iter().collect();

        let expected = vec![
            Argument::WithLiteral(
                Ident::new("value_one", Span::call_site()),
                "one".to_string(),
            ),
            Argument::Standalone(Ident::new("value_two", Span::call_site())),
            Argument::WithType(
                Ident::new("value_three", Span::call_site()),
                parse_quote! {
                    f32
                },
            ),
            Argument::WithType(
                Ident::new("value_four", Span::call_site()),
                parse_quote! {
                    four_real
                },
            ),
        ];

        assert_eq!(format!("{:?}", args), format!("{:?}", expected));
    }

    #[test]
    fn parse_as_type_option_attribute() {
        let item: ItemStruct = parse_quote! {
            struct MyItem {
                #[attr(value_one = Option<f32>)]
                field: i32
            }
        };
        let fields: Vec<Field> = item.fields.into_iter().collect();

        assert_eq!(fields.len(), 1);
        let field = fields.first().unwrap();

        assert_eq!(field.attrs.len(), 1);
        let attr = field.attrs.first().unwrap();

        let args = attr
            .parse_args_with(Punctuated::<Argument, Token![,]>::parse_terminated)
            .expect("failed to parse");
        let args: Vec<Argument> = args.into_iter().collect();

        let expected = vec![Argument::WithType(
            Ident::new("value_one", Span::call_site()),
            parse_quote! {
                Option<f32>
            },
        )];

        assert_eq!(format!("{:?}", args), format!("{:?}", expected));
    }

    #[test]
    fn parse_struct_operation_set() {
        let item: ItemStruct = parse_quote! {
            #[partially(rename = "PartialItem")]
            struct MyItem {
                field: i32
            }
        };

        let ctx = Context::from(&item);
        let item_set = OperationSet::new(&ctx, &item.attrs).expect("failed to parse OperationSet");

        assert_eq!(
            item_set.rename,
            Some(Ident::new("PartialItem", Span::call_site()))
        );
        assert!(item_set.as_type.is_none());
        assert!(!item_set.transparent);
    }

    #[test]
    fn parse_field_operation_set() {
        let item: ItemStruct = parse_quote! {
            struct MyItem {
                #[partially(rename = "number", as_type = f32)]
                field: i32
            }
        };
        let fields: Vec<Field> = item.fields.into_iter().collect();

        assert_eq!(fields.len(), 1);
        let field = fields.first().unwrap();

        let ctx = Context::from(field);
        let field_set =
            OperationSet::new(&ctx, &field.attrs).expect("failed to parse OperationSet");

        assert_eq!(
            field_set.rename,
            Some(Ident::new("number", Span::call_site()))
        );
        assert!(field_set.as_type.is_some());

        let expected_type: Type = parse_quote! {
            f32
        };
        assert_eq!(
            field_set.as_type.unwrap().into_token_stream().to_string(),
            expected_type.into_token_stream().to_string()
        );
        assert!(!field_set.transparent);
    }

    // implement `Context` for `ItemStruct` to make tests easier to author

    impl<'a> From<&'a ItemStruct> for Context<'a, ItemStruct> {
        fn from(value: &'a ItemStruct) -> Self {
            Self { item: value }
        }
    }

    impl<'a> ContextQuery for Context<'a, ItemStruct> {
        fn is_struct(&self) -> bool {
            true
        }

        fn is_field(&self) -> bool {
            false
        }
    }
}
