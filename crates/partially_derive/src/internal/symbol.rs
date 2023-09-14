use std::fmt::{self, Display};
use syn::{Ident, Path};

/// Represents a static literal that's comparable to common `syn` types.
#[derive(Copy, Clone)]
pub struct Symbol(&'static str);

/// The attribute name for the `#[derive]` attribute.
///
/// We use this to remove `Partial` from the derive attribute for the generated struct.
pub const DERIVE: Symbol = Symbol("derive");

/// The trait name for partial.
///
/// We use this to remove from `#[derive]` for the generated struct.
pub const PARTIAL: Symbol = Symbol("Partial");

/// The attribute name that scopes all operations.
///
/// Note: Operations are defined within the attribute as comma separated values:
/// `#[partially(rename = "to_this_name", as_type = f32)]`
/// `#[partially(transparent)]`
pub const PARTIALLY: Symbol = Symbol("partially");

/// The operation name for renaming the target.
///
/// Valid for `struct` and `field` members. Expects an `Ident` or `ExprLit` as the value.
pub const RENAME: Symbol = Symbol("rename");

/// The operation name for making the target transparent, meaning we will not make any changes to it's type.
///
/// Valid for `field` members. Expects no arguments.
pub const TRANSPARENT: Symbol = Symbol("transparent");

/// The operation name for changing the output type manually.
///
/// Valid for `field` members. Expects a `Type` as the value.
pub const AS_TYPE: Symbol = Symbol("as_type");

impl PartialEq<Symbol> for Ident {
    fn eq(&self, word: &Symbol) -> bool {
        self == word.0
    }
}

impl<'a> PartialEq<Symbol> for &'a Ident {
    fn eq(&self, word: &Symbol) -> bool {
        *self == word.0
    }
}

impl PartialEq<Symbol> for Path {
    fn eq(&self, word: &Symbol) -> bool {
        self.is_ident(word.0)
    }
}

impl<'a> PartialEq<Symbol> for &'a Path {
    fn eq(&self, word: &Symbol) -> bool {
        self.is_ident(word.0)
    }
}

impl Display for Symbol {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(self.0)
    }
}
