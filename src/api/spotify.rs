use base64::{Engine as _, engine::general_purpose};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SpotifyError {
    #[error("Environment variable error: {0}")]
    EnvVar(#[from] std::env::VarError),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Invalid token response: missing access_token")]
    InvalidTokenResponse,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
}

pub struct SpotifyClient {
    client: Client,
    access_token: String,
}

mod search {
    static SEARCH_TYPES: &[&str] = &["album", "artist", "playlist", "track"];
    static GENRES: &[&str] = &[
        "renaissance",
        "baroque",
        "classical",
        "romantic",
        "impressionism",
        "20th century classical",
        "contemporary classical",
    ];

    pub fn search_type() -> String {
        return SEARCH_TYPES.join(",");
    }
    pub fn genres() -> String {
        return GENRES.join(",");
    }
}

impl SpotifyClient {
    pub async fn new() -> Result<Self, SpotifyError> {
        let client_id = std::env::var("CLIENT_ID")?;
        let client_secret = std::env::var("CLIENT_SECRET")?;

        let credentials = format!("{}:{}", client_id, client_secret);
        let encoded = general_purpose::STANDARD.encode(credentials);

        let client = Client::new();
        let response = client
            .post("https://accounts.spotify.com/api/token")
            .header("Authorization", format!("Basic {}", encoded))
            .form(&[("grant_type", "client_credentials")])
            .send()
            .await?;

        let token_response: TokenResponse = response.json().await?;

        if token_response.access_token.is_empty() {
            return Err(SpotifyError::InvalidTokenResponse);
        }

        Ok(SpotifyClient {
            client,
            access_token: token_response.access_token,
        })
    }

    pub async fn simple_search(
        &self,
        raw_search: String,
    ) -> Result<serde_json::Value, SpotifyError> {
        let response = self
            .client
            .get("https://api.spotify.com/v1/search")
            .header("Authorization", format!("Bearer {}", self.access_token))
            .query(&[
                ("q", raw_search.as_str()),
                ("type", &search::search_type()),
                ("genre", &search::genres()),
                ("limit", "2"),
            ])
            .send()
            .await?;
        let result = response.json().await?;
        println!("{}", result);
        Ok(result)
    }
}
