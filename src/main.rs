mod repl;
mod api;

use thiserror::Error;

use crate::api::spotify::{SpotifyClient, SpotifyError};

#[derive(Error, Debug)]
enum AppError {
    #[error("Spotify error: {0}")]
    Spotify(#[from] SpotifyError),

    #[error("REPL error: {0}")]
    Repl(#[from] reedline_repl_rs::Error),
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let spotify = SpotifyClient::new().await?;

    let mut repl = repl::create_repl()?;
    repl.run().map_err(AppError::from)
}
