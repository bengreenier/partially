use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    parse_quote, punctuated::Punctuated, token::Comma, Data, DeriveInput, Field, Ident, Meta, Path,
    Result, Token,
};

use attr::OperationSet;
use ctx::Context;

use self::symbol::{DERIVE, PARTIAL, PARTIALLY};

mod attr;
mod ctx;
mod impl_partial;
mod symbol;

/// Expands the `#[derive(Partial)]` macro for the given input.
///
/// Note: Supports `#[partially()]` attributes on the struct and it's fields for further control.
pub fn expand_derive_partial(item: &mut DeriveInput) -> Result<TokenStream> {
    if !matches!(item.data, Data::Struct(_)) {
        return Err(syn::Error::new_spanned(
            item,
            "Partial is only supported for structs",
        ));
    }

    let root_ctx = Context::from(item as &DeriveInput);
    let root_attributes = OperationSet::new(&root_ctx, &item.attrs)?;

    let mut partial_item = item.clone();

    // process rename for the struct
    // note: other attributes are for fields only, so ignored here
    if let Some(name) = root_attributes.rename() {
        partial_item.ident = name;
    } else {
        let ident = &partial_item.ident;
        partial_item.ident = Ident::new(&format!("Partial{}", ident), Span::call_site())
    }

    // remove the `partially` attributes from the copied struct
    partial_item.attrs.retain(|a| a.path() != PARTIALLY);

    // TODO(bengreenier): there's a bug here preventing this from getting filled
    let mut derive_args: Punctuated<Path, Comma> = Punctuated::new();

    for attr in &mut partial_item.attrs {
        if attr.path() != DERIVE {
            continue;
        }

        let args = attr.parse_args_with(Punctuated::<Path, Token![,]>::parse_terminated)?;
        for path in args {
            if path != PARTIAL {
                derive_args.push_value(path);
            }
        }
    }

    return Err(syn::Error::new_spanned(
        item,
        format!("derive args is {:?}", derive_args.len()),
    ));

    if let Data::Struct(data) = &mut partial_item.data {
        for field in &mut data.fields {
            let field_ctx = Context::from(field as &Field);
            let field_attributes = OperationSet::new(&field_ctx, &field.attrs)?;

            // process any rename attempt
            if let Some(name) = field_attributes.rename() {
                if field.ident.is_none() {
                    return Err(syn::Error::new_spanned(
                        field,
                        "Cannot rename a tuple field",
                    ));
                } else {
                    field.ident = Some(name);
                }
            }

            // if as_type specifies a specific type, use that
            if let Some(ty) = field_attributes.as_type() {
                field.ty = ty;
            }
            // if no as_type and no transparent, replace with Option<original_type>
            else if !field_attributes.transparent() {
                let ty = &field.ty;

                field.ty = parse_quote! {
                    Option<#ty>
                };
            }

            // remove the `partially` attributes from the copied field
            field.attrs.retain(|a| a.path() != PARTIALLY);
        }

        // generate the output stream
        let out = quote! {
            #[derive(#derive_args)]
            #partial_item
        };

        // return the combined items
        Ok(out)
    } else {
        // the partial_item will always be a data_struct otherwise
        // the initial check against item would have bailed out early
        unreachable!()
    }
}

fn is_partial_derive_path(path: &Path) -> bool {
    if path == PARTIAL {
        return true;
    }

    if path.segments.len() != 2 {
        return false;
    }

    let first = &path.segments[0];
    let second = &path.segments[1];

    first.ident == PARTIALLY && second.ident == PARTIAL
}

#[cfg(test)]
mod test {
    use quote::ToTokens;
    use syn::{parse_quote, DeriveInput};

    use super::expand_derive_partial;

    #[test]
    fn basic_e2e() {
        let mut input: DeriveInput = parse_quote! {
            #[derive(partially::Partial, Default, Debug)]
            #[partially(rename = "PartialData")]
            #[some_attr]
            struct Data {
                /// A documented field.
                #[some_attr]
                str_field: String,
                #[partially(as_type = Option<f32>)]
                #[some_attr]
                number_field: i32,
                #[partially(transparent)]
                transparent_field: Option<String>,
                #[partially(rename = "new_field")]
                old_field: String
            }
        };

        let expanded = expand_derive_partial(&mut input).expect("failed to expand_derive_partial");

        let expected: DeriveInput = parse_quote! {
            #[derive(Default, Debug)]
            #[some_attr]
            struct PartialData {
                /// A documented field.
                #[some_attr]
                str_field: Option<String>,
                #[some_attr]
                number_field: Option<f32>,
                transparent_field: Option<String>,
                new_field: Option<String>
            }
        };

        assert_eq!(
            expanded.to_string(),
            expected.into_token_stream().to_string()
        );
    }

    #[test]
    fn dumb() {
        let ts: proc_macro2::TokenStream = parse_quote! {
            use partially::Partial as Fuck;

            #[attr(key = Fuck)]
            struct Test {

            }
        };

        println!("{:?}", ts);
    }
}
