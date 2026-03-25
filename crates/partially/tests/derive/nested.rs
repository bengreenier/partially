use partially::Partial;

#[derive(Debug, partially_derive::Partial)]
#[partially(derive(Default))]
struct Inner {
    v: String,
}

#[derive(Debug, partially_derive::Partial)]
#[partially(derive(Default))]
struct Outer {
    v: String,
    #[partially(nested)]
    inner: Inner,
}

#[test]
fn nested_apply_some() {
    let mut data = Outer {
        v: "v1".to_string(),
        inner: Inner {
            v: "v2".to_string(),
        },
    };

    let empty_partial = PartialOuter::default();
    let full_partial = PartialOuter {
        v: Some("v3".to_string()),
        inner: PartialInner {
            v: Some("v4".to_string()),
        },
    };

    assert!(!data.apply_some(empty_partial));
    assert_eq!(data.v, "v1");
    assert_eq!(data.inner.v, "v2");

    assert!(data.apply_some(full_partial));
    assert_eq!(data.v, "v3");
    assert_eq!(data.inner.v, "v4");
}

#[test]
fn nested_partial_to_partial() {
    let mut partial = PartialOuter::default();
    let update = PartialOuter {
        v: Some("updated".to_string()),
        inner: PartialInner {
            v: Some("nested_updated".to_string()),
        },
    };

    assert!(partial.apply_some(update));
    assert_eq!(partial.v, Some("updated".to_string()));
    assert_eq!(partial.inner.v, Some("nested_updated".to_string()));
}

#[test]
fn nested_only_inner_changes() {
    let mut data = Outer {
        v: "original".to_string(),
        inner: Inner {
            v: "original_inner".to_string(),
        },
    };

    let partial = PartialOuter {
        v: None,
        inner: PartialInner {
            v: Some("changed".to_string()),
        },
    };

    assert!(data.apply_some(partial));
    assert_eq!(data.v, "original");
    assert_eq!(data.inner.v, "changed");
}

#[derive(Debug, partially_derive::Partial)]
#[partially(derive(Default))]
struct Mixed {
    normal: String,
    #[partially(nested)]
    child: Inner,
    #[partially(transparent)]
    transparent_field: Option<String>,
    #[partially(omit)]
    _omitted: String,
}

#[test]
fn nested_with_other_field_options() {
    let mut data = Mixed {
        normal: "a".to_string(),
        child: Inner {
            v: "b".to_string(),
        },
        transparent_field: Some("c".to_string()),
        _omitted: "d".to_string(),
    };

    let partial = PartialMixed {
        normal: Some("x".to_string()),
        child: PartialInner {
            v: Some("y".to_string()),
        },
        transparent_field: Some("z".to_string()),
    };

    assert!(data.apply_some(partial));
    assert_eq!(data.normal, "x");
    assert_eq!(data.child.v, "y");
    assert_eq!(data.transparent_field, Some("z".to_string()));
    assert_eq!(data._omitted, "d");
}
