use partially::Partial;

#[derive(partially_derive::Partial)]
#[partially(derive(Default))]
struct Data<T> {
    value: T,
}

#[test]
fn generic_apply_some() {
    let empty_partial = PartialData::<String>::default();
    let full_partial = PartialData {
        value: Some("modified".to_string()),
    };

    let mut full = Data {
        value: "initial".to_string(),
    };

    full.apply_some(empty_partial);

    assert_eq!(full.value, "initial".to_string());

    full.apply_some(full_partial);

    assert_eq!(full.value, "modified".to_string());
}
