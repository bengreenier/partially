use partially::Partial;
use serde::{Deserialize, Serialize};
use serde_json::to_string;

#[derive(Partial)] // derive these on the base
#[partially(skip_attributes)] // don't automatically derive the same attributes as the base
#[partially(derive(Serialize, Deserialize))] // but, do derive these
#[partially(attribute(serde(rename_all = "SCREAMING_SNAKE_CASE")))] // add this attribute
#[partially(attribute(serde(default)))] // and this as well
struct Data {
    value: String,
}

impl Default for PartialData {
    fn default() -> Self {
        // not a very good default, but illustrates the point
        Self {
            value: Some("hello world".to_string()),
        }
    }
}

#[test]
fn basic_container_attrs() {
    let empty = PartialData::default();
    let full = PartialData {
        value: Some("some_value".to_string()),
    };

    assert_eq!(
        to_string(&empty).unwrap(),
        "{\"VALUE\":\"hello world\"}".to_string()
    );
    assert_eq!(
        to_string(&full).unwrap(),
        "{\"VALUE\":\"some_value\"}".to_string()
    );
}
