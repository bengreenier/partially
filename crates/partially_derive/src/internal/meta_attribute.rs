use std::ops::Deref;

use darling::FromMeta;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    Attribute, Meta,
};

/// An [`Attribute`] that supports being parsed with darling's [`FromMeta`].
///
/// Note: Only a single attribute value is supported.
#[derive(Debug, Clone)]
pub struct MetaAttribute {
    attr: Attribute,
}

/// A list of [`Attribute`] values, parsed with [`Attribute::parse_outer`].
struct OuterAttributeList(Vec<Attribute>);

impl Parse for OuterAttributeList {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self(input.call(Attribute::parse_outer)?))
    }
}

impl Deref for OuterAttributeList {
    type Target = Vec<Attribute>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromMeta for MetaAttribute {
    // note: errors must call [`darling::Error::with_span`] passing `item`
    fn from_meta(item: &Meta) -> darling::Result<Self> {
        // just parse as-if this is only Meta itself..
        // note: this _doesn't_ need the `map_err` logic, as it'll already have the correct span info
        let meta = Meta::from_meta(item)?;

        // ...but, require that it's a list, as it'll have a darling attribute name as it's path
        let meta = meta
            .require_list()
            .map_err(|e| Into::<darling::Error>::into(e).with_span(item))?
            .to_owned();

        let attr_tokens = &meta.tokens;
        let attr = quote! {
            #[#attr_tokens]
        };

        let attrs = syn::parse2::<OuterAttributeList>(attr)
            .map_err(|e| Into::<darling::Error>::into(e).with_span(item))?;

        // we can only support one item per `MetaAttribute`
        // despite `OuterAttributeList` supporting multiple
        if attrs.len() > 1 {
            return Err(darling::Error::too_many_items(1).with_span(item));
        }

        let attr = attrs[0].to_owned();

        Ok(Self { attr })
    }
}

impl ToTokens for MetaAttribute {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.attr.to_tokens(tokens)
    }
}

#[cfg(test)]
mod test {
    use darling::FromMeta;
    use quote::{quote, ToTokens};
    use syn::{parse_quote, Attribute};

    use super::MetaAttribute;

    #[test]
    fn single_attribute_path() {
        // note: `parse_quote` can't handle Meta directly, so we use `Attribute`.
        let input: Attribute = parse_quote!(#[ignored_name(attr_name)]);
        let input = MetaAttribute::from_meta(&input.meta).unwrap();

        let actual = input.to_token_stream().to_string();
        let expected = quote!(#[attr_name]).to_string();

        assert_eq!(actual, expected);
    }

    #[test]
    fn single_attribute_list() {
        // note: `parse_quote` can't handle Meta directly, so we use `Attribute`.
        let input: Attribute = parse_quote!(#[ignored_name(attr_name(attr_path))]);
        let input = MetaAttribute::from_meta(&input.meta).unwrap();

        let actual = input.to_token_stream().to_string();
        let expected = quote!(#[attr_name(attr_path)]).to_string();

        assert_eq!(actual, expected);
    }

    #[test]
    fn single_attribute_name_value() {
        // note: `parse_quote` can't handle Meta directly, so we use `Attribute`.
        let input: Attribute = parse_quote!(#[ignored_name(attr_name(value_name = "value_value"))]);
        let input = MetaAttribute::from_meta(&input.meta).unwrap();

        let actual = input.to_token_stream().to_string();
        let expected = quote!(#[attr_name(value_name = "value_value")]).to_string();

        assert_eq!(actual, expected);
    }

    #[test]
    fn multi_attribute_fail() {
        // note: `parse_quote` can't handle Meta directly, so we use `Attribute`.
        let input: Attribute =
            parse_quote!(#[ignored_name(attr_name1(attr_path1), attr_name2(attr_path2))]);

        assert!(MetaAttribute::from_meta(&input.meta).is_err());
    }
}
