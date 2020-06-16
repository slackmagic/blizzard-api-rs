pub mod character_equipment;
pub mod character_media;
pub mod character_profile;
pub mod character_statistics;

use crate::battle_net::oauth_token::OAuthToken;
use crate::wow::character_equipment::CharacterEquipment;
use crate::wow::character_media::CharacterMedia;
use crate::wow::character_profile::CharacterProfile;
use crate::wow::character_statistics::CharacterStatistics;
use crate::Settings;

#[derive(Debug)]
pub struct WowApi {}

impl WowApi {
    pub async fn character_profile(
        token: &OAuthToken,
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
        Ok(resp)
    }

    pub async fn character_media(
        token: &OAuthToken,
        server: &String,
        name: &String,
        settings: &Settings,
    ) -> Result<CharacterMedia, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/profile/wow/character/{}/{}/character-media?namespace={}&locale={}&access_token={}",
            WowApi::get_base_server_url(&settings.region),
            server,
            name,
            settings.namespace,
            settings.locale,
            token.access_token
        );

        let parsed_url = reqwest::Url::parse(&url)?;
        let resp = reqwest::get(parsed_url.as_ref()).await?.json().await?;

        Ok(resp)
    }

    pub async fn character_statistics(
        token: &OAuthToken,
        server: &String,
        name: &String,
        settings: &Settings,
    ) -> Result<CharacterStatistics, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/profile/wow/character/{}/{}/statistics?namespace={}&locale={}&access_token={}",
            WowApi::get_base_server_url(&settings.region),
            server,
            name,
            settings.namespace,
            settings.locale,
            token.access_token
        );

        let parsed_url = reqwest::Url::parse(&url)?;
        let resp = reqwest::get(parsed_url.as_ref()).await?;
        println!("{:?}", resp);
        let resp_json = resp.json().await?;
        println!("{:?}", resp_json);

        Ok(resp_json)
    }

    pub async fn character_equipment(
        token: &OAuthToken,
        server: &String,
        name: &String,
        settings: &Settings,
    ) -> Result<CharacterEquipment, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/profile/wow/character/{}/{}/equipment?namespace={}&locale={}&access_token={}",
            WowApi::get_base_server_url(&settings.region),
            server,
            name,
            settings.namespace,
            settings.locale,
            token.access_token
        );

        let parsed_url = reqwest::Url::parse(&url)?;
        let resp = reqwest::get(parsed_url.as_ref()).await?;
        println!("{:?}", resp);
        let resp_json = resp.json().await?;
        println!("{:?}", resp_json);
        Ok(resp_json)
    }

    fn get_base_server_url(region: &String) -> String {
        format!("https://{}.api.blizzard.com", region)
    }
}
