use partially::Partial;

#[derive(Partial)]
#[partially(derive(Default))]
struct Data {
    value: String,
}

#[test]
fn basic_apply_some() {
    let empty_partial = PartialData::default();
    let full_partial = PartialData {
        value: Some("modified".to_string()),
    };

    let mut full = Data {
        value: "initial".to_string(),
    };

    assert!(!full.apply_some(empty_partial));

    assert_eq!(full.value, "initial".to_string());

    assert!(full.apply_some(full_partial));

    assert_eq!(full.value, "modified".to_string());
}
