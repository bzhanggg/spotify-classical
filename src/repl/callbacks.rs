use reedline_repl_rs::clap::ArgMatches;
use std::result::Result;
use thiserror::Error;

use crate::api::spotify::{SpotifyClient, SpotifyError};

#[derive(Error, Debug)]
pub enum CallbackError {
    #[error("Spotify API Error: {0}")]
    ApiError(#[from] SpotifyError),

    #[error("REPL error: {0}")]
    Repl(#[from] reedline_repl_rs::Error),
}

pub async fn search(
    args: ArgMatches,
    spotify_client: &mut SpotifyClient,
) -> Result<Option<String>, CallbackError> {
    let search_string = args
        .get_many::<String>("search-string")
        .unwrap()
        .map(|s| s.as_str())
        .collect::<Vec<_>>()
        .join(" ");

    println!("Search for: {}", search_string);

    let results = spotify_client.simple_search(search_string).await?;
    Ok(Some(format!("{}", results)))
}
