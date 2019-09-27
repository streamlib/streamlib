extern crate clap;

use clap::{Arg, App, crate_authors, crate_version};

mod cli;
mod library;
mod player;
mod utils;

use cli::Selector;
use library::Library;
use player::Player;

fn main() {
    let matches = App::new("Streamlib")
        .version(crate_version!())
        .author(crate_authors!())
        .about("A video stream meta-player and specification")
        .arg(Arg::with_name("query")
            .help("TOML file library")
            .required(true)
            .index(1))
        .arg(Arg::with_name("library")
            .short("l")
            .long("library")
            .help("TOML file library")
            .takes_value(true))
        .arg(Arg::with_name("player")
            .short("p")
            .long("player")
            .help("Media player to run, defaults to `mpv`")
            .takes_value(true))
        .get_matches();

    let lib = Library::from_directory("library/**/*.toml");
    let q = matches.value_of("query").unwrap();
    let entry = Selector::from(lib).select(q);
    match entry {
        Some(e) => Player::from(e).play(),
        None => {
            println!("No match found...")
        }
    }
}
