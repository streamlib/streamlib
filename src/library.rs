extern crate toml;
extern crate serde;

use std::collections::BTreeMap;
use serde::Deserialize;

use super::utils;

#[derive(Clone, Deserialize, Debug)]
pub struct Entry {
    name: Option<String>,
    description: Option<String>,
    pub url: String,
    tags: Option<Vec<String>>
}

pub type LibraryEntries = BTreeMap<String, Entry>;

pub struct Library {
    pub entries: Vec<Entry>
}

impl Library {
    pub fn from_str(s: &str) -> Self {
        let entmap: LibraryEntries = toml::from_str(s).unwrap();
        let entries: Vec<Entry> = entmap.values().cloned().collect();
        Library {
            entries
        }
    }
    pub fn from_file(filename: &str) -> Self {
        let tomlstr = utils::open_file(filename);
        Library::from_str(tomlstr.as_str())
    }
}

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
        let lib = Library::from_str(TEST_LIB);
        assert_eq!(lib.entries[0].name, Some(String::from("Groove Salad")));
        assert_eq!(lib.entries[1].url, String::from("http://somafm.com/secretagent.pls"));
    }

    #[test]
    fn test_search() {
        let lib = Library::from_str(TEST_LIB);
        for e in lib.entries {
            assert_eq!(e.url.starts_with("http"), true);
        }
    }
}
