extern crate termion;

use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};
use std::{time, thread};

pub struct Selector {
    // screen: AlternateScreen<Stdout>,

}

impl Selector {
    pub fn new() -> Self {
        Selector {
            // screen: AlternateScreen::from(stdout())
        }
    }

    pub fn select(self) -> String {
        let stdin = stdin();
        let mut stdout = stdout();
        for i in {0..3} {
            write!(stdout, "{}This is a test... {}{}",
                // termion::clear::All,
                termion::cursor::Goto(i, 1),
                i,
                termion::cursor::Hide,
            ).unwrap();
            thread::sleep(time::Duration::from_secs(1));
            stdout.flush().unwrap();
        }
        String::from("http://somafm.com/groovesalad.pls")
    }
}
