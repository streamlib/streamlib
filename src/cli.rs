use std::io::{stdout, Write};
use std::{thread, time};
use termion::screen::*;
#[cfg(target_family = "unix")]
use termion::{color, style};

use super::library::{Entry, Library};

pub struct Selector {
    lib: Library,
}

impl Selector {
    pub fn from(lib: Library) -> Self {
        Selector { lib }
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
            #[cfg(target_family = "unix")]
            println!("{}{}{}{}{}\n\t{}\n", color::Fg(color::Yellow), style::Bold, name, style::Reset, desc, e.url);
            #[cfg(target_family = "windows")]
            println!("{}{}\n\t{}\n",name, desc, e.url);
        }
    }
}

#[cfg(target_family = "unix")]
pub fn start_gui() {
    {
        let mut screen = AlternateScreen::from(stdout());
        let message = r"

  _________ __                                .__  ._____.
 /   _____//  |________   ____ _____    _____ |  | |__\_ |__
 \_____  \\   __\_  __ \_/ __ \\__  \  /     \|  | |  || __ \
 /        \|  |  |  | \/\  ___/ / __ \|  Y Y  \  |_|  || \_\ \
/_______  /|__|  |__|    \___  >____  /__|_|  /____/__||___  /
        \/                   \/     \/      \/             \/

";
        write!(screen, "{}", message).unwrap();
        screen.flush().unwrap();

        thread::sleep(time::Duration::from_secs(2));
    }
    println!("This is still an experiment. Byeeeee");
}
