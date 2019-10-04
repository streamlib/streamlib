extern crate clap;
extern crate termion;

use clap::{Arg, App, crate_authors, crate_version};

mod cli;
mod git;
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
        .arg(Arg::with_name("list")
            .short("L")
            .long("list")
            .help("List all library entries")
            )
        .arg(Arg::with_name("query")
            .help("Stream name/description/URL to query")
            .required_unless("list")
            .index(1))
        .arg(Arg::with_name("library")
            .short("l")
            .long("library")
            .help("TOML file library")
            .takes_value(true))
        .arg(Arg::with_name("player")
            .short("p")
            .long("player")
            .default_value("mpv")
            .help("Media player to run, defaults to `mpv`")
            .takes_value(true))
        .get_matches();

    let lib = Library::from_directory("library/**/*.toml");

    if matches.is_present("list") {
        Selector::from(lib).list();
        return
    }

    let q = matches.value_of("query").unwrap();
    let player = matches.value_of("player").unwrap();
    let entry = Selector::from(lib).select(q);


    match entry {
        Some(e) => Player::from(e, player).play(),
        None => {
            println!("No match found...")
        }
    }
}
