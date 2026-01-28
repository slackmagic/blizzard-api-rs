pub mod oauth_token;
use crate::battle_net::oauth_token::OAuthToken;
use crate::Settings;
use reqwest::Client;

#[derive(Debug)]
pub struct BattleNetApi {}

impl BattleNetApi {
    pub async fn get_token(
        client: &String,
        secret: &String,
        settings: &Settings,
    ) -> Result<OAuthToken, Box<dyn std::error::Error>> {
        let endpoint = "/oauth/token";
        let url = format!(
            "{}{}",
            BattleNetApi::get_base_server_url(&settings.region),
            endpoint.to_owned()
        );

        let form = reqwest::multipart::Form::new()
            .text("grant_type", "client_credentials");

        let token: OAuthToken = Client::new()
            .post(&url)
            .basic_auth(client, Some(secret))
            .multipart(form)
            .send()
            .await?
            .json()
            .await?;

        Ok(token)
    }

    fn get_base_server_url(region: &String) -> String {
        format!("https://{}.battle.net", region)
    }
}
