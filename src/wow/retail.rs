use crate::battle_net::oauth_token::OAuthToken;
use crate::Settings;
use reqwest::Client;

// Modules WoW Retail
pub mod character;
pub mod statistics;
pub mod equipment;

// Re-export des types principaux
pub use character::*;
pub use statistics::*;
pub use equipment::*;

#[derive(Debug)]
pub struct WowRetailApiWrapper {}


impl WowRetailApiWrapper {
    pub async fn character_profile(
        token: &OAuthToken,
        server: &String,
        name: &String,
        settings: &Settings,
    ) -> Result<CharacterProfile, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/profile/wow/character/{}/{}?namespace={}&locale={}",
            WowRetailApiWrapper::get_base_server_url(&settings.region),
            server,
            name,
            settings.namespace,
            settings.locale
        );

        let parsed_url = reqwest::Url::parse(&url)?;
        let response = Client::new()
            .get(parsed_url)
            .bearer_auth(token.access_token.clone())
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            return Err(format!("API error {}: {}", status, text).into());
        }

        serde_json::from_str(&text).map_err(|e| {
            eprintln!("\n========================================");
            eprintln!("Failed to parse character_profile for {}", name);
            eprintln!("Error: {}", e);
            eprintln!("Response body length: {} bytes", text.len());
            eprintln!("Response body:\n{}", text);
            eprintln!("========================================\n");
            Box::new(e) as Box<dyn std::error::Error>
        })
    }

    pub async fn character_media(
        token: &OAuthToken,
        server: &String,
        name: &String,
        settings: &Settings,
    ) -> Result<CharacterMedia, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/profile/wow/character/{}/{}/character-media?namespace={}&locale={}",
            WowRetailApiWrapper::get_base_server_url(&settings.region),
            server,
            name,
            settings.namespace,
            settings.locale
        );

        let parsed_url = reqwest::Url::parse(&url)?;

        let resp = Client::new()
            .get(parsed_url)
            .bearer_auth(token.access_token.clone())
            .send()
            .await?
            .json()
            .await?;

        Ok(resp)
    }

    pub async fn character_statistics(
        token: &OAuthToken,
        server: &String,
        name: &String,
        settings: &Settings,
    ) -> Result<CharacterStatistics, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/profile/wow/character/{}/{}/statistics?namespace={}&locale={}",
            WowRetailApiWrapper::get_base_server_url(&settings.region),
            server,
            name,
            settings.namespace,
            settings.locale
        );

        let parsed_url = reqwest::Url::parse(&url)?;

        let resp = Client::new()
            .get(parsed_url)
            .bearer_auth(token.access_token.clone())
            .send()
            .await?
            .json()
            .await?;

        Ok(resp)
    }

    pub async fn character_equipment(
        token: &OAuthToken,
        server: &String,
        name: &String,
        settings: &Settings,
    ) -> Result<CharacterEquipment, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/profile/wow/character/{}/{}/equipment?namespace={}&locale={}",
            WowRetailApiWrapper::get_base_server_url(&settings.region),
            server,
            name,
            settings.namespace,
            settings.locale
        );

        let parsed_url = reqwest::Url::parse(&url)?;

        let response = Client::new()
            .get(parsed_url)
            .bearer_auth(token.access_token.clone())
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            return Err(format!("API error {}: {}", status, text).into());
        }

        serde_json::from_str(&text).map_err(|e| {
            eprintln!("Failed to parse character_equipment for {}: {}", name, e);
            eprintln!("Response body: {}", text);
            Box::new(e) as Box<dyn std::error::Error>
        })
    }

    fn get_base_server_url(region: &String) -> String {
        format!("https://{}.api.blizzard.com", region)
    }
}
