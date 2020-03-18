extern crate duct;
extern crate regex;
extern crate reqwest;

use duct::cmd;
use regex::Regex;
use std::collections::HashMap;

use super::library::{Entry, Query};
use super::smil::get_best_stream;
use super::utils::json_query;

pub struct Player {
    player: String,
    entry: Entry,
    debug: bool,
}

impl Player {
    pub fn from(entry: Entry, player: &str) -> Self {
        Player {
            player: String::from(player),
            entry: entry,
            debug: false,
        }
    }

    pub fn build_args(&self) -> Vec<String> {
        let mut args = vec![];

        if self.debug {
            args.push(String::from("-v"))
        }

        if self.entry.http_headers.is_some() {
            let headers = self.entry.http_headers.as_ref().unwrap();
            if headers.len() > 0 && self.player == "mpv" {
                // Headers are currently supported only with `mpv`
                let hs = headers.join(",");
                args.push(format!("--http-header-fields={}", hs));
            }
        }

        args.push(self.entry.url.clone());
        args
    }

    pub fn resolve_queries(&self, queries: &Vec<Query>) -> HashMap<String, String> {
        let mut args = HashMap::new();

        for query in queries {
            let url = query.url.as_str();
            println!("Fetching query arg {} from {}", query.name, url);
            let mut res = reqwest::get(url).expect(format!("Error calling {}", url).as_str());

            let mut val = String::new();

            if query.regex.is_some() {
                let re = Regex::new(&query.regex.as_ref().unwrap()).unwrap();
                let restext = &res.text().unwrap();
                let caps = re.captures(restext).unwrap();
                val.push_str(caps.get(1).unwrap().as_str());
            } else if query.json.is_some() {
                let jsonval: serde_json::Value = res.json().expect("Invalid json data");
                let finalval = json_query(&jsonval, &query.json.as_ref().unwrap());
                val.push_str(finalval.as_str());
            } else {
                println!(
                    "Query arg {} is missing a regex/json pattern and will probably fail...",
                    query.name
                );
            }

            args.insert(query.name.clone(), val);
        }
        args
    }

    pub fn build_url_query(&self, args: HashMap<String, String>) -> String {
        let mut url = self.entry.url.clone();
        let mut res = String::new();
        for (key, value) in args {
            res.push_str(key.as_str());
            res.push_str("=");
            res.push_str(value.as_str());
            res.push_str("&");
        }
        res.pop(); // remove the last ampersand
        if url.contains("?") {
            url.push_str("&");
        } else {
            url.push_str("?");
        }
        url.push_str(&res);
        url
    }

    pub fn play(&mut self, noplay: bool) {
        println!("Starting player process");
        let player = self.player.as_str();

        if self.entry.smil.unwrap_or(false) {
            println!("Handling SMIL url {}", self.entry.url);
            self.entry.url = get_best_stream(&self.entry.url);
        }

        if self.entry.query.is_some() {
            let queries = self.entry.query.as_ref().unwrap();
            if queries.len() > 0 {
                let query_args = self.resolve_queries(queries);
                self.entry.url = self.build_url_query(query_args);
            }
        }

        let args = self.build_args();

        if !noplay {
            match cmd(player, args).run() {
                Ok(_) => {
                    println!("Player process terminated");
                }
                Err(e) => {
                    println!("Player {} not found, please install it or use a custom player with `-p playername` ({})", player, e);
                }
            }
        } else {
            println!("Skipping actual stream play...");
            println!("{} {:?}", player, args);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::library::Entry;
    use super::super::utils::json_query;
    use super::Player;
    use std::collections::HashMap;

    #[test]
    fn test_http_headers_args() {
        let mut entry = Entry::from_url(String::from("http://example.com/"));
        entry.http_headers = Some(vec![String::from("A: b"), String::from("C: d")]);
        let p = Player {
            player: String::from("mpv"),
            entry: entry,
            debug: false,
        };
        assert_eq!(
            p.build_args(),
            ["--http-header-fields=A: b,C: d", "http://example.com/"]
        );
    }

    #[test]
    fn test_no_http_headers_args() {
        let mut entry = Entry::from_url(String::from("http://example.com/"));
        entry.http_headers = Some(vec![String::from("A: b"), String::from("C: d")]);
        let p = Player {
            player: String::from("cvlc"),
            entry: entry,
            debug: false,
        };
        assert_eq!(p.build_args().len(), 1); // just the url
    }

    #[test]
    fn test_debug_args() {
        let entry = Entry::from_url(String::from("http://example.com/"));
        let p = Player {
            player: String::from("mpv"),
            entry: entry,
            debug: true,
        };
        assert_eq!(p.build_args(), ["-v", "http://example.com/"]);
    }

    #[test]
    fn test_player_not_found() {
        let entry = Entry::from_url(String::from("http://example.com/"));
        let mut p = Player {
            player: String::from("nonexistentplayer"),
            entry: entry,
            debug: false,
        };
        // will throw an error if not caught
        p.play(false);
    }

    #[test]
    fn test_http() {
        let resp: serde_json::Value = reqwest::get("https://mass.mako.co.il/ClicksStatistics/entitlementsServicesV2.jsp?et=ngt&lp=/hls/live/512033/CH2LIVE_HIGH/index.m3u8&rv=AKAMAI").unwrap().json().unwrap();
        let jq = "tickets.0.ticket";
        println!("{}", json_query(&resp, jq));
    }

    #[test]
    fn test_append_args() {
        let entry = Entry::from_url(String::from("http://example.com/feed.m3u8"));
        let p = Player {
            player: String::from("nonexistentplayer"),
            entry: entry,
            debug: false,
        };
        let mut args = HashMap::new();
        args.insert(String::from("abc"), String::from("def"));
        let url = p.build_url_query(args);

        assert_eq!(url, "http://example.com/feed.m3u8?abc=def")
    }

    #[test]
    fn test_append_args_with_existing() {
        let entry = Entry::from_url(String::from("http://example.com/feed.m3u8?old=args"));
        let p = Player {
            player: String::from("nonexistentplayer"),
            entry: entry,
            debug: false,
        };
        let mut args = HashMap::new();
        args.insert(String::from("abc"), String::from("def"));
        let url = p.build_url_query(args);

        assert_eq!(url, "http://example.com/feed.m3u8?old=args&abc=def")
    }
}
