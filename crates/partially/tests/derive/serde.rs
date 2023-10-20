use serde::Deserialize;
use partially::Partial;

const EMPTY_INPUT: &str = "{}";
const FILLED_INPUT: &str = "{ \"value\": \"modified\" }";

#[derive(Partial)]
#[partially(serde, derive(Default, Deserialize))]
struct Data {
    value: String,
}

#[test]
fn serde_derive() {
    let empty_partial: PartialData = serde_json::from_str(EMPTY_INPUT).unwrap();
    let full_partial: PartialData = serde_json::from_str(FILLED_INPUT).unwrap();

    let mut full = Data {
        value: "initial".to_string(),
    };

    full.apply_some(empty_partial);

    assert_eq!(full.value, "initial".to_string());

    full.apply_some(full_partial);

    assert_eq!(full.value, "modified".to_string());
}
