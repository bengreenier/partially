use partially_derive::Partial;

/// A test struct.
#[derive(Partial, Default, Debug)]
#[partially(rename = "PartialTest")]
struct Test {
    /// Some test number.
    #[partially(as_type = Option<f32>)]
    number: i32,
}

#[test]
fn can_parse() {
    let a = Test::default();
    let b = PartialTest { number: Some(1.0) };

    println!("a: {:?}, b: {:?}", a, b);
}
