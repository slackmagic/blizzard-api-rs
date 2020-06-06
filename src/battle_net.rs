pub mod oauth_token;
use crate::battle_net::oauth_token::OAuthToken;
use crate::Settings;
use reqwest::blocking::Client;
use reqwest::Client as AsyncClient;

#[derive(Debug)]
pub struct BattleNetApi {}

impl BattleNetApi {
    #[tokio::main]
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

        println!("{}", url);

        let req_client: AsyncClient = AsyncClient::new();

        //Request
        let token: OAuthToken = req_client
            .post(&url)
            .basic_auth(client, Some(secret))
            .form(&[("grant_type", "client_credentials")])
            .send()
            .await?
            .json()
            .await?;

        Ok(token)
    }

    pub fn get_token_blocking(
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

        println!("{}", url);

        let req_client: reqwest::blocking::Client = reqwest::blocking::Client::new();

        //Request
        let res: OAuthToken = req_client
            .post(&url)
            .basic_auth(client, Some(secret))
            .form(&[("grant_type", "client_credentials")])
            .send()?
            .json()?;

        Ok(res)
    }

    fn get_base_server_url(region: &String) -> String {
        format!("https://{}.battle.net", region)
    }
}
