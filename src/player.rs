extern crate duct;
extern crate reqwest;
extern crate serde_json;

use duct::cmd;
use std::collections::HashMap;
use super::library::{Entry, Query};

pub struct Player {
    player: String,
    url: String,
    headers: Vec<String>,
    queries: Vec<Query>,
    debug: bool
}

impl Player {
    pub fn from(entry: Entry, player: &str) -> Self {
        Player {
            player: String::from(player),
            url: entry.url,
            headers: entry.http_headers.unwrap_or(vec![]),
            queries: entry.query.unwrap_or(vec![]),
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

        if self.queries.len() > 0 {

        }

        args.push(self.url.clone());
        args
    }

    pub fn append_query_args(&mut self, args: HashMap<&str, &str>) {
        let mut res = String::new();
        for (key, value) in args {
            res.push_str(key);
            res.push_str("=");
            res.push_str(value);
            res.push_str("&");
        }
        res.pop(); // remove the last ampersand
        if self.url.contains("?") {
            self.url.push_str("&");
        }
        else {
            self.url.push_str("?");
        }
        self.url.push_str(&res);
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
    use std::collections::HashMap;
    use super::Player;
    use super::super::utils::json_query;

    #[test]
    fn test_http_headers_args() {
        let p = Player {
            player: String::from("mpv"),
            url: String::from("http://example.com/"),
            headers: vec![String::from("A: b"), String::from("C: d")],
            queries: vec![],
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
            queries: vec![],
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
            queries: vec![],
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
            queries: vec![],
            debug: false
        };
        // will throw an error if not caught
        p.play();
    }

    #[test]
    fn test_http() {
        let resp: serde_json::Value = reqwest::get("https://mass.mako.co.il/ClicksStatistics/entitlementsServicesV2.jsp?et=ngt&lp=/hls/live/512033/CH2LIVE_HIGH/index.m3u8&rv=AKAMAI").unwrap().json().unwrap();
        let jq = "tickets.0.ticket";
        println!("{}", json_query(&resp, jq));
    }

    #[test]
    fn test_append_args() {
        let mut p = Player {
            player: String::from("nonexistentplayer"),
            url: String::from("http://example.com/feed.m3u8"),
            headers: vec![],
            queries: vec![],
            debug: false
        };
        let mut args = HashMap::new();
        args.insert("abc", "def");
        p.append_query_args(args);

        assert_eq!(p.url, "http://example.com/feed.m3u8?abc=def")
    }

    #[test]
    fn test_append_args_with_existing() {
        let mut p = Player {
            player: String::from("nonexistentplayer"),
            url: String::from("http://example.com/feed.m3u8?old=args"),
            headers: vec![],
            queries: vec![],
            debug: false
        };
        let mut args = HashMap::new();
        args.insert("abc", "def");
        p.append_query_args(args);

        assert_eq!(p.url, "http://example.com/feed.m3u8?old=args&abc=def")
    }
}
