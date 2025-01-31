use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, Scope, TokenResponse, TokenUrl};
use reqwest::{redirect, ClientBuilder};

use super::cli::AppConfig;

pub async fn get_auth_header(config: &AppConfig) -> anyhow::Result<String> {
    let auth_client = BasicClient::new(ClientId::new(config.client_id.clone()))
        .set_client_secret(ClientSecret::new(config.client_secret.clone()))
        .set_auth_uri(AuthUrl::new(config.auth_url.clone())?)
        .set_token_uri(TokenUrl::new(config.token_url.clone())?);

    let http_client = ClientBuilder::new()
        .redirect(redirect::Policy::none())
        .build()?;

    let token_result = auth_client
        .exchange_client_credentials()
        .add_scope(Scope::new(config.scope.clone()))
        .request_async(&http_client)
        .await?;

    Ok(format!("Bearer {}", token_result.access_token().secret()))
}
