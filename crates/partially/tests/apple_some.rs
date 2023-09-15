use partially::Partial;

struct Base {
    value: String,
}

#[derive(Default)]
struct PartialBase {
    value: Option<String>,
}

impl partially::Partial for Base {
    type Item = PartialBase;

    #[allow(clippy::useless_conversion)]
    fn apply_some(&mut self, partial: Self::Item) {
        if let Some(value) = partial.value {
            self.value = value.into();
        }
    }
}

#[test]
fn basic_apply() {
    let empty_partial = PartialBase::default();
    let full_partial = PartialBase {
        value: Some("modified".to_string()),
    };

    let mut data = Base {
        value: "initial".to_string(),
    };

    data.apply_some(empty_partial);

    assert_eq!(data.value, "initial".to_string());

    data.apply_some(full_partial);

    assert_eq!(data.value, "modified".to_string())
}
