pub mod commands;

use crate::api::spotify::SpotifyClient;

use reedline_repl_rs::clap::{Arg, Command};
use reedline_repl_rs::{Error, Repl, Result};

pub fn create_repl(spotify_client: SpotifyClient) -> Result<Repl<SpotifyClient, Error>> {
    let repl = Repl::new(spotify_client)
        .with_name("SpotifyClassical")
        .with_version("v0.0.1")
        .with_description("Better classical music discovery on Spotify")
        .with_banner("Welcome to SpotifyClassical! Type 'help' to see available commands.")
        .with_command_async(
            Command::new("search").arg(
                Arg::new("search-string")
                    .help("Search for a piece, composer, etc.")
                    .num_args(1..)
                    .required(true),
            ),
            |args, context| Box::pin(commands::search::<SpotifyClient>(args, context)),
        );
    Ok(repl)
}
