use quote::{quote, ToTokens};

/// Separators for use with [`TokenVec`].
#[allow(unused)]
pub enum Separator {
    /// Value of ` `.
    Space,
    /// Value of `,`.
    Comma,
    /// Value of `\n`.
    Newline,
    /// Value of `,\n`.
    CommaNewline,
    /// Value of `::`.
    ColonColon,
}

impl Default for Separator {
    fn default() -> Self {
        Self::Space
    }
}

/// Storage for a set of [`ToTokens`].
pub struct TokenVec<T: ToTokens> {
    inner: Vec<T>,
    separator: Separator,
}

impl<T: ToTokens> TokenVec<T> {
    /// Creates a new [`TokenVec`] from a [`Vec<T>`] using the default [`Separator`].
    pub fn new_with_vec(inner: Vec<T>) -> Self {
        Self::new_with_vec_and_sep(inner, Separator::default())
    }

    /// Creates a new [`TokenVec`] from a [`Vec<T>`] using the given [`Separator`].
    pub fn new_with_vec_and_sep(inner: Vec<T>, separator: Separator) -> Self {
        Self { inner, separator }
    }
}

impl<T: ToTokens> From<Vec<T>> for TokenVec<T> {
    fn from(value: Vec<T>) -> Self {
        Self::new_with_vec(value)
    }
}

impl<T: ToTokens> ToTokens for TokenVec<T> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let len = self.inner.len();
        for (index, token) in self.inner.iter().enumerate() {
            // serialize each token and it's separator
            match &self.separator {
                // never use a separator for the final token
                _ if index == len - 1 => tokens.extend(quote!(#token)),

                Separator::Space => tokens.extend(quote!(#token)),
                Separator::Comma => tokens.extend(quote!(#token,)),
                Separator::ColonColon => tokens.extend(quote!(#token::)),
                Separator::Newline => tokens.extend(quote! {
                        #token
                }),
                Separator::CommaNewline => tokens.extend(quote! {
                    #token,
                }),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Separator, TokenVec};
    use quote::{quote, ToTokens};

    #[test]
    fn separates_space() {
        let instance =
            TokenVec::new_with_vec_and_sep(vec![quote!(a), quote!(b), quote!(c)], Separator::Space);

        let actual = instance.into_token_stream().to_string();

        let expected = quote!(a b c);
        let expected = expected.into_token_stream().to_string();

        assert_eq!(actual, expected);
    }

    #[test]
    fn separates_comma() {
        let instance =
            TokenVec::new_with_vec_and_sep(vec![quote!(a), quote!(b), quote!(c)], Separator::Comma);

        let actual = instance.into_token_stream().to_string();

        let expected = quote!(a, b, c);
        let expected = expected.into_token_stream().to_string();

        assert_eq!(actual, expected);
    }

    #[test]
    fn separates_colon_colon() {
        let instance = TokenVec::new_with_vec_and_sep(
            vec![quote!(a), quote!(b), quote!(c)],
            Separator::ColonColon,
        );

        let actual = instance.into_token_stream().to_string();

        let expected = quote!(a::b::c);
        let expected = expected.into_token_stream().to_string();

        assert_eq!(actual, expected);
    }

    #[test]
    fn separates_newline() {
        let instance = TokenVec::new_with_vec_and_sep(
            vec![quote!(a), quote!(b), quote!(c)],
            Separator::Newline,
        );

        let actual = instance.into_token_stream().to_string();

        let expected = quote! {
            a
            b
            c
        };
        let expected = expected.into_token_stream().to_string();

        assert_eq!(actual, expected);
    }

    #[test]
    fn separates_comma_newline() {
        let instance = TokenVec::new_with_vec_and_sep(
            vec![quote!(a), quote!(b), quote!(c)],
            Separator::CommaNewline,
        );

        let actual = instance.into_token_stream().to_string();

        let expected = quote! {
            a,
            b,
            c
        };
        let expected = expected.into_token_stream().to_string();

        assert_eq!(actual, expected);
    }
}
