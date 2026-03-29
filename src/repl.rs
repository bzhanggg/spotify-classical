pub mod commands;

use reedline_repl_rs::clap::{Arg, Command};
use reedline_repl_rs::{Repl, Result, Error};

pub fn create_repl() -> Result<Repl<(), Error>> {
    let repl = Repl::new(())
        .with_name("SpotifyClassical")
        .with_version("v0.0.1")
        .with_description("Better classical music discovery on Spotify")
        .with_banner("Welcome to SpotifyClassical! Type 'help' to see available commands.")
        .with_command(
            Command::new("search")
                .arg(Arg::new("search-string")
                    .help("Search for a piece, composer, etc.")
                    .num_args(1..)
                    .required(true)),
            commands::search
        );
    Ok(repl)
}
