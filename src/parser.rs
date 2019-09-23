extern crate toml;
extern crate serde;

use std::collections::BTreeMap;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Entry {
    name: Option<String>,
    description: Option<String>,
    url: String,
    tags: Option<Vec<String>>
}

type Library = BTreeMap<String, Entry>;

#[cfg(test)]
mod tests {
    use super::Library;

    const TEST_LIB: &'static str = r#"
        [groovesalad]
        name = "Groove Salad"
        description = "A nicely chilled plate of ambient/downtempo beats and grooves"
        url = "http://somafm.com/groovesalad.pls"
        tags = ["somafm", "radio", "ambient", "groove"]

        [secretagent]
        url = "http://somafm.com/secretagent.pls"
        "#;

    #[test]
    fn test_struct_parse() {
        let lib: Library = toml::from_str(TEST_LIB).unwrap();
        assert_eq!(lib.get("groovesalad").unwrap().name, Some(String::from("Groove Salad")));
        assert_eq!(lib.get("secretagent").unwrap().url, String::from("http://somafm.com/secretagent.pls"));
    }
}
