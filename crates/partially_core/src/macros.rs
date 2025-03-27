/// Creates a named type that wraps a `String`.
#[macro_export]
macro_rules! string_proxy {
    ($type:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct $type(pub String);

        // if we're in codegen mode we know we can implement quote::ToTokens easily
        // so we just do it in the macro for "free".
        #[cfg(feature = "codegen")]
        impl quote::ToTokens for $type {
            fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
                tokens.extend(quote::quote! { #self });
            }
        }

        impl std::str::FromStr for $type {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(Self(s.to_owned()))
            }
        }

        impl From<String> for $type {
            fn from(s: String) -> Self {
                Self(s)
            }
        }

        impl From<&str> for $type {
            fn from(s: &str) -> Self {
                Self(s.to_owned())
            }
        }

        impl std::ops::Deref for $type {
            type Target = String;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::ops::DerefMut for $type {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}
