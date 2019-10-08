extern crate glob;
extern crate serde;
extern crate toml;

use glob::glob;
use std::collections::BTreeMap;
use serde::Deserialize;

use super::utils;

#[derive(Clone, Deserialize, Debug)]
pub struct Entry<'a> {
    pub name: Option<&'a str>,
    pub description: Option<&'a str>,
    pub url: &'a str,
    pub tags: Option<Vec<&'a str>>,
    pub http_headers: Option<Vec<&'a str>>
}

pub type LibraryEntries<'a> = BTreeMap<String, Entry<'a>>;

pub struct Library<'a> {
    pub entries: Vec<Entry<'a>>
}

impl<'de, T> Library<'de> {
    pub fn from_str(s: &str) -> Self {
        let entmap: LibraryEntries = toml::from_str(s).unwrap();
        let entries: Vec<Entry> = entmap.values().cloned().collect();
        Self {
            entries
        }
    }

    pub fn from_file(filename: &str) -> Self {
        let tomlstr = utils::open_file(filename);
        Library::from_str(tomlstr.as_str())
    }

    pub fn from_directory(dir: &str) -> Self {
        let mut lib = Library {
            entries: vec![]
        };

        println!("{}", dir);
        let dirglob = format!("{}/**/*.toml", dir);
        for entry in glob(dirglob.as_str()).expect("Failed to read glob pattern") {
            let e = entry.unwrap();
            let mut entries = Self::from_file(e.to_str().unwrap()).entries;
            lib.entries.append(&mut entries);
        }

        println!("Loaded {} stream entries...", lib.entries.len());
        lib
    }

    pub fn query(&self, q: &str) -> Vec<Entry> {
        self.entries
            .iter()
            .filter(|e| {
                let n = e.name.clone().unwrap_or(String::new()).to_ascii_lowercase();
                let d = e.description.clone().unwrap_or(String::new()).to_ascii_lowercase();
                let u = e.url.to_ascii_lowercase();
                n.contains(q) || d.contains(q) || u.contains(q)
            })
            .cloned()
            .collect()
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
        http_headers = ["Header-Name: value123", "Foo: goo"]

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

    #[test]
    fn test_query() {
        let lib = Library::from_str(TEST_LIB);
        assert_eq!(lib.query("groove").len(), 1);
        assert_eq!(lib.query("secret").len(), 1);
        assert_eq!(lib.query("chilled").len(), 1);
        assert_eq!(lib.query("soma").len(), 2);
        assert_eq!(lib.query("nomatch").len(), 0);
    }
}
