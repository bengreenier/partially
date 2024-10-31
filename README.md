# partially

> Provides a configurable [derive macro](https://doc.rust-lang.org/reference/procedural-macros.html#derive-macros) that generates another struct with the same fields, but wrapped in `Option<T>`, and implements `Partial` to allow conditionally applying the generated struct fields to the base struct fields.

`partially` provides the `Partial` trait, which allows applying another structs values to `self`, with the intent that the other struct mirrors the fields of `self`, but wrapped in `Option<T>`.

Further, `partially_derive` (or `partially` with the `derive` feature enabled) supports automatically generating a mirrored struct with each field wrapped in `Option<T>`, **and** generates a `Partial` implementation that allows applying the `Some` fields of the mirrored struct to the base struct. I expect most folks will be most interested in using the derive macro.

## Usage

With derive:

```rust
// `partially` installed with feature `derive`
use partially::Partial;

// define a base structure, with the `Partial` derive macro
#[derive(Partial)]
// further, instruct the macro to derive `Default` on the generated structure
#[partially(derive(Default))]
struct Data {
    // since no field options are specified, this field will be mapped
    // to an `Option<String>` in the generated structure
    value: String,
}

// example usage
fn main() {
    // since we derived default for the generated struct, we can use that
    // to obtain a partial struct filled with `None`.
    let empty_partial = PartialData::default();

    // we can, of course, also specify values ourself
    let full_partial = PartialData {
        value: Some("modified".to_string()),
    };

    // define a "base" that we'll operate against
    let mut full = Data {
        value: "initial".to_string(),
    };

    // apply the empty partial (note that `false` is returned, indicating nothing was applied)
    assert!(!full.apply_some(empty_partial));

    // note that applying the empty partial had no effect
    assert_eq!(full.value, "initial".to_string());

    // apply the full partial (note that `true` is returned, indicating something was applied)
    assert!(full.apply_some(full_partial));

    // note that applying the full partial modified the value
    assert_eq!(full.value, "modified".to_string());
}
```

Without derive:

```rust
use partially::Partial;

struct Base {
    value: String,
}

#[derive(Default)]
struct PartialBase {
    value: Option<String>,
}

impl Partial for Base {
    type Item = PartialBase;

    #[allow(clippy::useless_conversion)]
    fn apply_some(&mut self, partial: Self::Item) -> bool {
        let will_apply_some = partial.value.is_some();

        if let Some(value) = partial.value {
            self.value = value.into();
        }

        will_apply_some
    }
}

fn main() {
    let empty_partial = PartialBase::default();
    let full_partial = PartialBase {
        value: Some("modified".to_string()),
    };

    let mut data = Base {
        value: "initial".to_string(),
    };

    assert!(!data.apply_some(empty_partial));

    assert_eq!(data.value, "initial".to_string());

    assert!(data.apply_some(full_partial));

    assert_eq!(data.value, "modified".to_string())
}

```

### Struct Options

#### derive

> Usage example: `#[partially(derive(Debug, Default))]`.

Instructs the macro to generate a `#[derive(...)]` attribute on the generated struct.

Note: When using this option with the `skip_attributes` option, the derive attribute **will still be added to the generated struct**.

#### rename

> Usage example: `#[partially(rename = "MyGeneratedStruct")]`.

Instructs the macro to use a given identifier for the generated struct. By default, `PartialBaseStructName` is used, where `BaseStructName` is the name of the original struct.

#### attribute

> Usage example: `#[partially(attribute(serde(rename_all = "PascalCase")))]`

Instructs the macro to add an additional attribute to the generated struct. By default, the attributes defined on the base struct are forwarded to the generated struct, unless the `skip_attributes` option is present.

#### skip_attributes

> Usage example: `#[partially(skip_attributes)]`.

Instructs the macro to skip forwarding attributes from the original struct to the generated struct. By default, all attributes that are present on the base struct are added to the generated struct.

Note: When using this option with the `derive` option, the derive attribute **will still be added to the generated struct**.

Note: When using this option with the `attribute` option, the specified attribute(s) **will still be added to the generated struct**.

#### crate

> Usage example: `#[partially(crate = "my_partially_crate")]`.

Instructs the macro to use a different base path for the `Partial` trait implementation. By default, `partially` is used. This can be useful if you've forked the `partially` crate.

### Field Options

#### rename

> Usage example: `#[partially(rename = "new_field_name")]`.

Instructs the macro to use a given identifier for the generated field. By default, the same name as the base struct is used.

#### omit

> Usage example: `#[partially(omit)]`.

Instructs the macro to omit the field from the generated struct. By default, no fields are omitted.

#### transparent

> Usage example: `#[partially(transparent)]`.

Instructs the macro to skip wrapping the generated field in `Option<T>`, instead transparently mirroring the field type into the generated struct.

#### as_type

> Usage example: `#[partially(as_type = "Option<f32>")]`.

Instructs the macro to use the provided type instead of `Option<T>` when generating the field. Note that the provided type will be used verbatim, so if you expect an `Option<T>` value, you'll need to manually specify that.

Note: When using `as_type`, the given type must `Into<BaseType>` where `BaseType` is the original field type. This is required for `Partial` trait implementation.

#### nested

> Usage example: `#[partially(nested)]`

Indicates to the macro that this field is `Partial`. The type in the generated struct will then be the associated `Partial::Item` type of the field's type and the struct's `Partial::apply_some` implementation will call `Partial::apply_some` on the field.