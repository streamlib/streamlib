#[cfg(not(target_os = "windows"))]
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
        self.lib.query(q).pop()
    }

    pub fn list(self, q: &str) {
        for e in self.lib.query(q) {
            let name = e.name.unwrap_or(String::from("(untitled)"));
            let mut desc = e.description.unwrap_or(String::new());
            if desc.len() > 0 {
                desc = format!(" - {}", desc);
            }
            #[cfg(not(target_os = "windows"))]
            println!("{}{}{}{}{}\n\t{}\n", color::Fg(color::Yellow), style::Bold, name, style::Reset, desc, e.url);
        }
    }
}
