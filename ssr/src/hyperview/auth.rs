use crate::AppConfig;
use anyhow::Result;
use oauth2::basic::BasicClient;
use oauth2::reqwest;
use oauth2::{AuthUrl, ClientId, ClientSecret, Scope, TokenResponse, TokenUrl};

pub fn get_auth_header(config: &AppConfig) -> Result<String> {
    let auth_client = BasicClient::new(ClientId::new(config.client_id.clone()))
        .set_client_secret(ClientSecret::new(config.client_secret.clone()))
        .set_auth_uri(AuthUrl::new(config.auth_url.clone())?)
        .set_token_uri(TokenUrl::new(config.token_url.clone())?);

    let http_client = reqwest::blocking::ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::none())
        .build()?;

    let token_result = auth_client
        .exchange_client_credentials()
        .add_scope(Scope::new("HyperviewManagerApi".to_string()))
        .request(&http_client)?;

    Ok(format!("Bearer {}", token_result.access_token().secret()))
}
