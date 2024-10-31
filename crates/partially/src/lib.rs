// support `no_std` environments
#![no_std]
// include our readme docs
#![doc = include_str!(concat!("../", env!("CARGO_PKG_README")))]

/// ## partially_derive
/// supports automatically generating a mirrored struct with each field wrapped in [`Option<T>`], and generates a `partially::Partial` implementation that allows applying the [`Some`] fields of the mirrored struct to the base struct.
///
/// ## Struct Options
/// ### derive
/// > Usage example: `#[partially(derive(Debug, Default))]`.
/// Instructs the macro to generate a `#[derive(...)]` attribute on the generated struct.
/// ### rename
/// > Usage example: `#[partially(rename = "MyGeneratedStruct")]`.
/// Instructs the macro to use a given identifier for the generated struct. By default, `Partial<BaseStructName>` is used.
/// ### attribute
/// > Usage example: `#[partially(attribute(serde(rename_all = "PascalCase")))]`
/// Instructs the macro to add an additional attribute to the generated struct. By default, the attributes defined on the base struct are forwarded to the generated struct, unless the `skip_attributes` option is present.
/// ### skip_attributes
/// > Usage example: `#[partially(skip_attributes)]`.
/// Instructs the macro to skip forwarding attributes from the original struct to the generated struct. By default, all attributes that are present on the base struct are added to the generated struct.
/// Note: When using this option with the `derive` option, the derive attribute **will still be added to the generated struct**.
/// Note: When using this option with the `attribute` option, the specified attribute(s) **will still be added to the generated struct**.
/// ### crate
/// > Usage example: `#[partially(crate = "my_partially_crate")]`.
/// Instructs the macro to use a different base path for the `Partial` trait implementation. By default, `partially` is used. This can be useful if you've forked the `partially` crate.
///
/// ## Field Options
/// ### rename
/// > Usage example: `#[partially(rename = "new_field_name")]`.
/// Instructs the macro to use a given identifier for the generated field. By default, the same name as the base struct is used.
/// ### omit
/// > Usage example: `#[partially(omit)]`.
/// Instructs the macro to omit the field from the generated struct. By default, no fields are omitted.
/// ### transparent
/// > Usage example: `#[partially(transparent)]`.
/// Instructs the macro to skip wrapping the generated field in [`Option<T>`], instead transparently mirroring the field type into the generated struct.
/// ### as_type
/// > Usage example: `#[partially(as_type = "Option<f32>")]`.
/// Instructs the macro to use the provided type instead of [`Option<T>`] when generating the field. Note that the provided type will be used verbatim, so if you expect an [`Option<T>`] value, you'll need to manually specify that.
/// Note: When using `as_type`, the given type must `Into<BaseType>` where `BaseType` is the original field type. This is required for `Partial` trait implementation.
/// ### nested
/// > Usage example: `#[partially(nested)]`
/// Indicates to the macro that this field is [`Partial`]. The type in the generated struct will then be the associated [`Partial::Item`] type of the field's type and the struct's [`Partial::apply_some`] implementation will call [`Partial::apply_some`] on the field.
///
/// ## Example
/// ```
/// use partially::Partial;
///
/// #[derive(Partial)]
/// #[partially(derive(Default))]
/// struct Data {
///     value: String,
/// }

/// let empty_partial = PartialData::default();
/// let full_partial = PartialData {
///     value: Some("modified".to_string()),
/// };

/// let mut full = Data {
///     value: "initial".to_string(),
/// };

/// full.apply_some(empty_partial);

/// assert_eq!(full.value, "initial".to_string());

/// full.apply_some(full_partial);

/// assert_eq!(full.value, "modified".to_string());
///
/// ```
#[cfg(feature = "derive")]
pub use partially_derive::Partial;

/// Allows applying a [`Partial::Item`] to `Self`, where [`Partial::Item`] has [`Some`] values.
pub trait Partial {
    /// The type of the partial structure, that may have [`Some`] values.
    type Item;

    /// Applies [`Some`] values from [`Partial::Item`] to [`self`], returning `true` when
    /// updates were made, and `false` when nothing was applied.
    ///
    /// Note: [`None`] values should not be applied.
    fn apply_some(&mut self, partial: Self::Item) -> bool;
}
