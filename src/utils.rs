use std::fs::File;
use std::io::prelude::*;

pub fn open_file(filename: &str) -> String {
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

/// This method allows querying a serde JSON value with a simple query syntax.
/// The query must be given as a dot (`.`) delimited string which contains
/// either string keys for querying objects, or numerical values for querying arrays
/// e.g. `phones.0` will return the first element in the `phones` array.
/// Only Strings are supported as return values.
pub fn json_query(json: &serde_json::Value, query: &str) -> String {
    let items = query.split(".");
    let mut res = json;
    let err = format!("Failed to query '{}' on {}", query, json);
    for item in items {
        res = match res.is_array() {
            true => &res.get(item.parse::<usize>().unwrap()).expect(&err),
            false => &res.get(item).expect(&err),
        };
    }
    let s = res.as_str().unwrap();
    String::from(s)
}

mod tests {
    #[cfg(test)]
    use super::json_query;

    #[test]
    fn test_json_query() {
        let data = r#"
            {
                "name": "John Doe",
                "age": 43,
                "phones": [
                    "+44 1234567",
                    "+44 2345678"
                ]
            }"#;
        let v = serde_json::from_str(data).unwrap();
        assert_eq!(json_query(&v, "name"), "John Doe");
        assert_eq!(json_query(&v, "phones.0"), "+44 1234567");
        assert_eq!(json_query(&v, "phones.1"), "+44 2345678");
    }
}
