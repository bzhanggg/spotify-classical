mod api;
mod repl;

use crate::api::{SpotifyClient, SpotifyError};
use crate::repl::callbacks::CallbackError;
use reedline_repl_rs::Repl;
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
enum AppError {
    #[error("Spotify error: {0}")]
    Spotify(#[from] SpotifyError),

    #[error("REPL error: {0}")]
    Repl(#[from] reedline_repl_rs::Error),
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let spotify_client = SpotifyClient::new().await?;

    let mut repl: Repl<SpotifyClient, CallbackError> = repl::create_repl(spotify_client)?;
    repl.run_async().await?;
    Ok(())
}
