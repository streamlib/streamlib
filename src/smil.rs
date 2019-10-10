extern crate quick_xml;

use quick_xml::Reader;
use quick_xml::events::Event;

fn build_url(url: String, servers: Vec<String>) -> String {
    let parts: Vec<&str> = url.split("://").collect();

    let schema = parts.get(0).unwrap();
    let mut urlparts: Vec<&str> = parts.get(1).unwrap().split("/").collect();
    urlparts[0] = servers[0].as_str();

    format!("{}://{}", schema, urlparts.join("/"))
}

pub fn get_best_stream(url: &String) -> String {
    let mut res = reqwest::get(url).expect(format!("Error calling {}", url).as_str());
    let restext = &res.text().unwrap();

    let mut reader = Reader::from_str(restext);
    reader.trim_text(true);

    let mut buf = Vec::new();
    let mut feed = String::new();
    let mut servers = Vec::new();

    const FEED_TAG_NAME: &[u8] = b"SmilURL";
    const SERVER_TAG_NAME: &[u8] = b"Server";

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => match e.name() {
                FEED_TAG_NAME => {
                    feed = reader.read_text(FEED_TAG_NAME, &mut Vec::new()).expect("Cannot decode feed value");
                }
                SERVER_TAG_NAME => {
                    servers.push(reader.read_text(SERVER_TAG_NAME, &mut Vec::new()).expect("Cannot decode server value"));
                }
                _ => ()
            }
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }

    build_url(feed, servers)
}
