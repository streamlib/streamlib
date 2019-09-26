extern crate duct;

use duct::cmd;
use super::library::Entry;

pub struct Player {
    url: String,
    headers: Vec<String>,
    debug: bool
}

impl Player {
    pub fn new(entry: Entry) -> Self {
        Player {
            url: entry.url,
            headers: entry.http_headers.unwrap_or(vec![]),
            debug: false
        }
    }

    pub fn play(self) {
        let mut args = vec![];

        if self.debug {
            args.push(String::from("-v"))
        }

        if self.headers.len() > 0 {
            args.push(String::from("--http-header-fields"));
            args.push(self.headers.join("','"));
        }

        args.push(self.url);

        println!("Starting mpv process");
        cmd("mpv", args).run().unwrap();
        println!("mpv process terminated");
    }
}
