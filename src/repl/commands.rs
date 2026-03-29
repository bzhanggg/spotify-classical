use reedline_repl_rs::Result;
use reedline_repl_rs::clap::ArgMatches;

use crate::api::spotify::SpotifyClient;

pub async fn search<T>(
    args: ArgMatches,
    spotify_client: &mut SpotifyClient,
) -> Result<Option<String>> {
    let search_string = args
        .get_many::<String>("search-string")
        .unwrap()
        .map(|s| s.as_str())
        .collect::<Vec<_>>()
        .join(" ");

    println!("Search for: {}", search_string);
    let results = spotify_client
        .simple_search(search_string)
        .await
        .map_err(|e| reedline_repl_rs::Error::IllegalDefaultError(e.to_string()))?;
    Ok(Some(format!("{}", results)))
}
