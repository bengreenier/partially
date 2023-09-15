use partially::Partial;

struct V1 {
    full_name: String,
}
struct V2 {
    first_name: String,
    last_name: Option<String>,
}

impl From<V1> for V2 {
    fn from(value: V1) -> Self {
        let parts = value.full_name.split_once(' ');

        let (first_name, last_name) = if let Some(parts) = parts {
            (parts.0.to_string(), Some(parts.1.to_string()))
        } else {
            (value.full_name, None)
        };

        Self {
            first_name,
            last_name,
        }
    }
}

#[derive(partially_derive::Partial)]
#[partially(derive(Default))]
struct Data {
    #[partially(as_type = "Option<V1>")]
    value: V2,
}

#[test]
fn retyped_apply_some() {
    let empty_partial = PartialData::default();
    let full_partial = PartialData {
        value: Some(V1 {
            full_name: "John Doe".to_string(),
        }),
    };

    let mut full = Data {
        value: V2 {
            first_name: "Sara".to_string(),
            last_name: Some("Smith".to_string()),
        },
    };

    full.apply_some(empty_partial);

    assert_eq!(full.value.first_name, "Sara".to_string());
    assert_eq!(full.value.last_name, Some("Smith".to_string()));

    full.apply_some(full_partial);

    assert_eq!(full.value.first_name, "John".to_string());
    assert_eq!(full.value.last_name, Some("Doe".to_string()));
}
