extern crate duct;

use duct::cmd;

pub struct Player {
    url: String
}

impl Player {
    pub fn new(url: String) -> Self {
        Player {
            url
        }
    }

    pub fn play(self) {
        println!("Starting mpv process");
        cmd!("mpv", self.url).run().unwrap();
        println!("mpv process terminated");
    }
}
