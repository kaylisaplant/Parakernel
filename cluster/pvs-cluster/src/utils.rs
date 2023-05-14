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