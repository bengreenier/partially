use partially::Partial;

#[derive(partially_derive::Partial)]
#[partially(derive(Default))]
struct Inner {
    value: String,
}

#[derive(partially_derive::Partial)]
#[partially(derive(Default))]
struct Outer {
    top: String,
    inner: Inner,
}

#[test]
fn nested_apply_some() {
    let mut outer = Outer {
        top: "top-initial".to_string(),
        inner: Inner {
            value: "inner-initial".to_string(),
        },
    };

    assert!(!outer.apply_some(PartialOuter::default()));
    assert_eq!(outer.top, "top-initial".to_string());
    assert_eq!(outer.inner.value, "inner-initial".to_string());

    let nested_only = PartialOuter {
        top: None,
        inner: PartialInner {
            value: Some("inner-updated".to_string()),
        },
    };

    assert!(outer.apply_some(nested_only));
    assert_eq!(outer.top, "top-initial".to_string());
    assert_eq!(outer.inner.value, "inner-updated".to_string());
}

#[derive(partially_derive::Partial)]
#[partially(derive(Default))]
struct GenericOuter<T> {
    value: T,
}

#[derive(partially_derive::Partial)]
#[partially(derive(Default))]
struct Wrapper {
    generic: GenericOuter<String>,
}

#[test]
fn generic_nested_apply_some() {
    let mut wrapper = Wrapper {
        generic: GenericOuter {
            value: "before".to_string(),
        },
    };

    let patch = PartialWrapper {
        generic: PartialGenericOuter {
            value: Some("after".to_string()),
        },
    };

    assert!(wrapper.apply_some(patch));
    assert_eq!(wrapper.generic.value, "after".to_string());
}

#[derive(partially_derive::Partial)]
#[partially(derive(Default))]
struct ExplicitOverridesStillWin {
    #[partially(as_type = "Option<String>")]
    inner: Inner,
}

#[test]
fn explicit_as_type_still_applies() {
    let mut value = ExplicitOverridesStillWin {
        inner: Inner {
            value: "before".to_string(),
        },
    };

    let patch = PartialExplicitOverridesStillWin {
        inner: Some("after".to_string()),
    };

    assert!(value.apply_some(patch));
    assert_eq!(value.inner.value, "after".to_string());
}
