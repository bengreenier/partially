use syn::{Data, DeriveInput, Field};

/// Trait that allows querying a context in an item-generic way.
pub trait ContextQuery {
    /// Determines if the context is for a struct.
    fn is_struct(&self) -> bool;

    /// Determines if the context is for a field.
    fn is_field(&self) -> bool;
}

/// Generic Context that provides [`ContextQuery`] for common `syn` types.
pub struct Context<'a, T> {
    pub item: &'a T,
}

impl<'a> From<&'a DeriveInput> for Context<'a, DeriveInput> {
    fn from(value: &'a DeriveInput) -> Self {
        Self { item: value }
    }
}

impl<'a> ContextQuery for Context<'a, DeriveInput> {
    fn is_struct(&self) -> bool {
        matches!(self.item.data, Data::Struct(_))
    }

    fn is_field(&self) -> bool {
        false
    }
}

impl<'a> From<&'a Field> for Context<'a, Field> {
    fn from(value: &'a Field) -> Self {
        Self { item: value }
    }
}

impl<'a> ContextQuery for Context<'a, Field> {
    fn is_struct(&self) -> bool {
        false
    }

    fn is_field(&self) -> bool {
        true
    }
}
