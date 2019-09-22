extern crate clap;
use clap::{Arg, App};

mod parser;

fn main() {
    let matches = App::new("Streamlib")
        .version("0.1")
        .author("Yuval Adam")
        .about("A video stream meta-player and specification")
        .arg(Arg::with_name("library")
            .help("TOML file library")
            .required(true)
            .index(1))
        .arg(Arg::with_name("player")
            .short("p")
            .long("player")
            .help("Media player to run, defaults to `mpv`")
            .takes_value(true))
        .get_matches();

    println!("{}", matches.value_of("player").unwrap_or("mpv"));
    println!("{}", matches.value_of("library").unwrap());
}
