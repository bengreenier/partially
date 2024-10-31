use partially::Partial;

#[derive(Debug, partially_derive::Partial)]
#[partially(derive(Default))]
struct Data2 {
    v: String,
}

#[derive(Debug, partially_derive::Partial)]
#[partially(derive(Default))]
struct Data1 {
    v: String,
    #[partially(nested)]
    d2: Data2,
}


#[test]
fn nested_apply_some() {
    let mut data = Data1 {
        v: "v1".to_string(),
        d2: Data2 {
            v: "v2".to_string(),
        },
    };

    let empty_partial = PartialData1::default();
    let full_partial = PartialData1 {
        v: Some("v3".to_string()),
        d2: PartialData2 {
            v: Some("v4".to_string()),
        },
    };

    data.apply_some(empty_partial);
    assert_eq!(data.v, "v1");
    assert_eq!(data.d2.v, "v2");

    data.apply_some(full_partial);
    assert_eq!(data.v, "v3");
    assert_eq!(data.d2.v, "v4");
}
