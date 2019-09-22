extern crate toml;

#[cfg(test)]
mod tests {
    use toml::Value;

    #[test]
    fn test_parse() {
        let val = r#"
        title = "SomaFm Radio Stations"

        [groovesalad]
        name = "Groove Salad"
        description = "A nicely chilled plate of ambient/downtempo beats and grooves"
        url = "http://somafm.com/groovesalad.pls"
        tags = ["somafm", "radio", "ambient", "groove"]
        "#.parse::<Value>().unwrap();

        assert_eq!(val["title"].as_str(), Some("SomaFm Radio Stations"));
    }
}
