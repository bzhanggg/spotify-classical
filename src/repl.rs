pub mod callbacks;

use crate::api::spotify::SpotifyClient;
use crate::repl::callbacks::CallbackError;
use reedline_repl_rs::clap::{Arg, Command};
use reedline_repl_rs::{Repl, Result};

pub fn create_repl(spotify_client: SpotifyClient) -> Result<Repl<SpotifyClient, CallbackError>> {
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
            |args, context| Box::pin(callbacks::search(args, context)),
        );
    Ok(repl)
}
