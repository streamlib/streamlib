extern crate duct;

use duct::cmd;
use super::library::Entry;

pub struct Player {
    url: String,
    headers: Vec<String>,
    debug: bool
}

impl Player {
    pub fn from(entry: Entry) -> Self {
        Player {
            url: entry.url,
            headers: entry.http_headers.unwrap_or(vec![]),
            debug: false
        }
    }

    pub fn build_args(self) -> Vec<String> {
        let mut args = vec![];

        if self.debug {
            args.push(String::from("-v"))
        }

        if self.headers.len() > 0 {
            args.push(String::from("--http-header-fields"));
            args.push(self.headers.join("','"));
        }

        args.push(self.url);
        args
    }

    pub fn play(self) {
        println!("Starting mpv process");
        cmd("mpv", self.build_args()).run().unwrap();
        println!("mpv process terminated");
    }
}


#[cfg(test)]
mod tests {
    use super::Player;

    #[test]
    fn test_http_headers_args() {
        let p = Player {
            url: String::from("http://example.com/"),
            headers: vec![String::from("A: b"), String::from("C: d")],
            debug: false
        };
        assert_eq!(p.build_args(), ["--http-header-fields", "A: b','C: d", "http://example.com/"]);
    }

    #[test]
    fn test_debug_args() {
        let p = Player {
            url: String::from("http://example.com/"),
            headers: vec![],
            debug: true
        };
        assert_eq!(p.build_args(), ["-v", "http://example.com/"]);
    }

}
