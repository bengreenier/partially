#[cfg(feature = "derive")]
pub use partially_derive::Partial;

pub trait Partial {
    type Item;

    fn apply_some(&mut self, partial: Self::Item);
}

#[derive(Default)]
pub struct TestBase {
    a: String,
}

#[derive(Default)]
pub struct PartialTestBase {
    a: Option<String>,
}

impl Partial for TestBase {
    type Item = PartialTestBase;

    fn apply_some(&mut self, partial: Self::Item) {
        if let Some(a) = partial.a {
            self.a = a;
        }
    }
}

fn test() {
    let mut base = TestBase::default();
    let partial = PartialTestBase::default();

    base.apply_some(partial);
}
