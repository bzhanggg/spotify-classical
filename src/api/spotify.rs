use base64::{engine::general_purpose, Engine as _};
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
    pub expires_in: u64
}

pub struct SpotifyClient {
    client: Client,
    access_token: String
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

        Ok(SpotifyClient {
            client,
            access_token: token_response.access_token
        })
    }
}