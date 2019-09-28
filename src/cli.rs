use termion::{color, style};

use super::library::{Entry, Library};

pub struct Selector {
    lib: Library
}

impl Selector {
    pub fn from(lib: Library) -> Self {
        Selector {
            lib
        }
    }

    pub fn select(self, q: &str) -> Option<Entry> {
        for e in self.lib.entries {
            let n = e.name.clone().unwrap_or(String::new()).to_ascii_lowercase();
            let d = e.description.clone().unwrap_or(String::new()).to_ascii_lowercase();
            let u = e.url.to_ascii_lowercase();
            let q = String::from(q.to_ascii_lowercase());
            let q = q.as_str();
            if n.contains(q) || d.contains(q) || u.contains(q) {
                return Some(e)
            }
        }
        None
    }

    pub fn list(self) {
        for e in self.lib.entries {
            let name = e.name.unwrap_or(String::from("(untitled)"));
            let mut desc = e.description.unwrap_or(String::new());
            if desc.len() > 0 {
                desc = format!(" - {}", desc);
            }
            println!("{}{}{}{}{}\n\t{}\n", color::Fg(color::Yellow), style::Bold, name, style::Reset, desc, e.url);
        }
    }
}
