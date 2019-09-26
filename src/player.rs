extern crate duct;

use duct::cmd;
use super::library::Entry;

pub struct Player {
    url: String,
    header: String
}

impl Player {
    pub fn new(entry: Entry) -> Self {
        Player {
            url: entry.url,
            header: entry.http_header.unwrap_or(String::new())
        }
    }

    pub fn play(self) {
        println!("Starting mpv process");
        cmd!("mpv", "--http-header-fields", self.header, self.url).run().unwrap();
        println!("mpv process terminated");
    }
}
