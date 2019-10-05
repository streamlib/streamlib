extern crate duct;

use duct::cmd;
use super::library::Entry;

pub struct Player {
    player: String,
    url: String,
    headers: Vec<String>,
    debug: bool
}

impl Player {
    pub fn from(entry: Entry, player: &str) -> Self {
        Player {
            player: String::from(player),
            url: entry.url,
            headers: entry.http_headers.unwrap_or(vec![]),
            debug: false
        }
    }

    pub fn build_args(&self) -> Vec<String> {
        let mut args = vec![];

        if self.debug {
            args.push(String::from("-v"))
        }

        if self.headers.len() > 0 && self.player == "mpv" {
            // Headers are currently supported only with `mpv`
            args.push(String::from("--http-header-fields"));
            args.push(self.headers.join("','"));
        }

        args.push(self.url.clone());
        args
    }

    pub fn play(&self) {
        println!("Starting player process");
        let player = self.player.as_str();
        let args = self.build_args();

        match cmd(player, args).run() {
            Ok(_) => {
                println!("Player process terminated");
            },
            Err(e) => {
                println!("Player {} not found, please install it or use a custom player with `-p playername` ({})", player, e);
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::Player;

    #[test]
    fn test_http_headers_args() {
        let p = Player {
            player: String::from("mpv"),
            url: String::from("http://example.com/"),
            headers: vec![String::from("A: b"), String::from("C: d")],
            debug: false
        };
        assert_eq!(p.build_args(), ["--http-header-fields", "A: b','C: d", "http://example.com/"]);
    }

    #[test]
    fn test_no_http_headers_args() {
        let p = Player {
            player: String::from("cvlc"),
            url: String::from("http://example.com/"),
            headers: vec![String::from("A: b"), String::from("C: d")],
            debug: false
        };
        assert_eq!(p.build_args().len(), 1); // just the url
    }

    #[test]
    fn test_debug_args() {
        let p = Player {
            player: String::from("mpv"),
            url: String::from("http://example.com/"),
            headers: vec![],
            debug: true
        };
        assert_eq!(p.build_args(), ["-v", "http://example.com/"]);
    }

    #[test]
    fn test_player_not_found() {
        let p = Player {
            player: String::from("nonexistentplayer"),
            url: String::from("http://example.com/"),
            headers: vec![],
            debug: false
        };
        // will throw an error if not caught
        p.play();
    }
}
