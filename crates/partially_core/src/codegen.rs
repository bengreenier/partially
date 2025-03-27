use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

use crate::{
    FieldConfig, FieldFlag, GenerationConfig, StructConfig, StructGeneric, Type, Visibility,
};

impl ToTokens for Visibility {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            Visibility::Private => quote!(),
            Visibility::Pub => quote!(pub),
            Visibility::PubCrate => quote!(pub(crate)),
        });
    }
}

impl ToTokens for FieldConfig {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let FieldConfig {
            name,
            visibility,
            ty,
            flag,
            target_name,
            target_ty,
        } = &self;

        let target_name = target_name.as_ref().unwrap_or(name);
        let target_ty = if flag == &FieldFlag::Transparent {
            ty
        } else if let Some(target_ty) = target_ty {
            target_ty
        } else {
            &Type(format!("Option<{0}>", ty.as_str()))
        };

        tokens.extend(quote!(#visibility #target_name: #target_ty));
    }
}

impl ToTokens for StructConfig {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let StructConfig {
            visibility,
            generic,
            fields,
            derives,
            source_attributes,
            additional_attributes,
            ..
        } = &self;

        // Build derives if any exist
        let derives = if !derives.is_empty() {
            quote!(#[derive(#(#derives),*)])
        } else {
            quote!()
        };

        // Include source attributes if not skipped
        let source_attrs = if !source_attributes.is_empty() {
            let filtered_attributes = source_attributes
                .iter()
                .filter(|a| !a.starts_with("partially"));

            quote!(#(#filtered_attributes)*)
        } else {
            quote!()
        };

        // Add any additional attributes
        let additional_attrs = quote!(#(#additional_attributes)*);

        // Get safe target name
        let target_name = self.safe_target_name();

        // Generate the struct definition with its fields
        let struct_def = match generic {
            Some(StructGeneric { ty, .. }) => {
                quote! {
                    #derives
                    #source_attrs
                    #additional_attrs
                    #visibility struct #target_name<#ty> {
                        #(#fields,)*
                    }
                }
            }
            _ => {
                quote! {
                    #derives
                    #source_attrs
                    #additional_attrs
                    #visibility struct #target_name {
                        #(#fields,)*
                    }
                }
            }
        };

        tokens.extend(struct_def);
    }
}

impl ToTokens for GenerationConfig {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let GenerationConfig { krate, structs } = self;

        for st in structs {
            st.to_tokens(tokens);

            let StructConfig {
                name,
                generic,
                fields,
                detailed_reporting,
                skipped_fields,
                ..
            } = st;

            let target_name = st.safe_target_name();

            let StructGeneric { ty, imp, wher } =
                generic.as_ref().unwrap_or(&StructGeneric::DEFAULT);

            tokens.extend(quote! {
                #st

                impl #imp #krate::Partial for #target_name #ty #wher {
                    type Item = #name #ty;

                    fn apply_some(&mut self, partial: Self::Item) -> bool {
                        let will_apply_some = #field_is_somes;

                        #field_applicators

                        will_apply_some
                    }
                }
            });
        }
    }
}
