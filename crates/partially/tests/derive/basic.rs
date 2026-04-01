use partially::Partial;

#[derive(Partial)]
#[partially(derive(Default))]
struct Data {
    a: String,
    b: String,
}

#[test]
fn from_full_maps_all_some() {
    let full = Data {
        a: "a".to_string(),
        b: "b".to_string(),
    };
    let partial: PartialData = full.into();
    assert_eq!(partial.a, Some("a".to_string()));
    assert_eq!(partial.b, Some("b".to_string()));
}

#[test]
fn skip_from_full() {
    #[derive(Partial)]
    #[partially(skip_from_full)]
    #[allow(unused)]
    struct Struct {
        a: String,
    }
    // NB: Without `skip_from_full` this would complain about a duplicate impl
    impl From<Struct> for PartialStruct {
        fn from(_: Struct) -> Self {
            unimplemented!()
        }
    }
}

#[test]
fn basic_apply_some() {
    let empty_partial = PartialData::default();
    let full_partial = PartialData {
        a: Some("modified".to_string()),
        b: None,
    };

    let mut full = Data {
        a: "initial".to_string(),
        b: String::default(),
    };

    assert!(!full.apply_some(empty_partial));

    assert_eq!(full.a, "initial".to_string());

    assert!(full.apply_some(full_partial));

    assert_eq!(full.a, "modified".to_string());
}

#[test]
fn partial_apply_some() {
    let mut empty_partial = PartialData::default();
    let full_partial = PartialData {
        a: Some("modified".to_string()),
        b: None,
    };

    assert!(empty_partial.apply_some(full_partial));

    assert_eq!(empty_partial.a, Some("modified".to_string()));
    assert_eq!(empty_partial.b, None);
}
