pub mod character_profile;

use crate::battle_net::oauth_token::OAuthToken;
use crate::wow::character_profile::CharacterProfile;
use crate::Settings;
use crate::BLIZZARD_SERVER;
use reqwest::Client;

#[derive(Debug)]
pub struct WowApi {}

impl WowApi {
    #[tokio::main]
    pub async fn character_profile(
        token: OAuthToken,
        server: &String,
        name: &String,
        settings: &Settings,
    ) -> Result<CharacterProfile, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/profile/wow/character/{}/{}?namespace={}&locale={}&access_token={}",
            WowApi::get_base_server_url(&settings.region),
            server,
            name,
            settings.namespace,
            settings.locale,
            token.access_token
        );

        let parsed_url = reqwest::Url::parse(&url)?;

        let resp = reqwest::get(parsed_url.as_ref()).await?.json().await?;
        println!("THE RESPONSE : {:?}", resp);
        Ok(resp)
    }

    #[tokio::main]
    pub async fn character_statistics(
        token: OAuthToken,
        server: &String,
        name: &String,
        settings: &Settings,
    ) {
        let endpoint = "/oauth/token";
    }

    #[tokio::main]
    pub async fn account_summary(token: OAuthToken, settings: &Settings) {
        let endpoint = "/profile/user/wow";
    }

    fn get_base_server_url(region: &String) -> String {
        format!("https://{}.api.blizzard.com", region)
    }

    fn parse_url(url: &String) {
        println!("{:?}", reqwest::Url::parse(url));
    }
}
