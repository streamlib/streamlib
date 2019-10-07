extern crate clap;
#[cfg(not(target_os = "windows"))]
extern crate termion;

use clap::{Arg, App, crate_authors, crate_version};

mod cli;
mod git;
mod library;
mod player;
mod utils;

use cli::Selector;
use git::Git;
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
            .help("Path to custom TOML file library")
            .takes_value(true))
        .arg(Arg::with_name("player")
            .short("p")
            .long("player")
            .help("Name of custom player executable")
            .takes_value(true))
        .arg(Arg::with_name("update")
            .short("u")
            .long("update")
            .help("Force a library update"))
        .get_matches();

    // Load the default library from https://github.com/streamlib/library
    // Or use an explicit path to a local directory
    let libpath = match matches.value_of("library") {
        Some(library) => String::from(library),
        None => {
            let force_update = matches.is_present("update");
            let git = Git::new(force_update);
            git.get_path()
        }
    };
    let lib = Library::from_directory(libpath.as_str());
    let selector = Selector::from(lib);
    let query = matches.value_of("query").unwrap_or("");

    // If we're just listing entries, print them and return
    if matches.is_present("list") {
        selector.list(query);
        return
    }

    // Otherwise, get the player and entry and run everything
    let player = matches.value_of("player").unwrap_or("mpv");
    let entry = selector.select(query);

    match entry {
        Some(e) => Player::from(e, player).play(),
        None => {
            println!("No match found...")
        }
    }
}
