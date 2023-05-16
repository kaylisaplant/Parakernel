use std::time::{SystemTime, UNIX_EPOCH};


pub fn only_or_error(vec: & Vec<String>) -> & String {
    match vec.as_slice() {
        [element] => element,
        _ => panic!("Vector does not contain a single element"),
    }
}

pub fn only_or_none(vec: & Vec<String>) -> Option<& String> {
    match vec.as_slice() {
        [element] => Some(element),
        _ => None,
    }
}

pub fn epoch() -> u64 {
    let now = SystemTime::now();
    let epoch = now.duration_since(UNIX_EPOCH).expect(
        "Failed to calculate duration"
    );

    epoch.as_secs()
}