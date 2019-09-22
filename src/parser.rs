extern crate toml;
extern crate serde;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Entry {
    name: Option<String>,
    description: Option<String>,
    url: String,
    tags: Vec<String>
}

#[derive(Deserialize, Debug)]
struct Library {
    title: Option<String>,
    entries: Vec<Entry>
}

#[cfg(test)]
mod tests {
    use toml::Value;
    use super::Library;

    const TEST_LIB: &'static str = r#"
        title = "SomaFm Radio Stations"

        [groovesalad]
        name = "Groove Salad"
        description = "A nicely chilled plate of ambient/downtempo beats and grooves"
        url = "http://somafm.com/groovesalad.pls"
        tags = ["somafm", "radio", "ambient", "groove"]
        "#;

    #[test]
    fn test_parse() {
        let val = TEST_LIB.parse::<Value>().unwrap();
        assert_eq!(val["title"].as_str(), Some("SomaFm Radio Stations"));
        let m = val.as_table().unwrap();
        for v in m.iter() {
            println!("{}", v.0);
        }
    }

    #[test]
    fn test_struct_parse() {
        // let val: Library = toml::from_str(TEST_LIB).unwrap();
    }
}
