pub fn get_best_stream(url: &String) -> String {
    let mut res = reqwest::get(url).expect(format!("Error calling {}", url).as_str());
    String::from(url)
}
