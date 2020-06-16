extern crate serde;
#[macro_use]
extern crate serde_derive;

pub mod battle_net;
pub mod wow;

use crate::battle_net::oauth_token::OAuthToken;
use crate::battle_net::BattleNetApi;
use crate::wow::character_equipment::CharacterEquipment;
use crate::wow::character_media::CharacterMedia;
use crate::wow::character_profile::CharacterProfile;
use crate::wow::character_statistics::CharacterStatistics;
use crate::wow::WowApi;

#[derive(Debug)]
pub struct Settings {
    region: String,
    namespace: String,
    locale: String,
}

impl Settings {
    pub fn new(region: String, namespace: String, locale: String) -> Settings {
        Settings {
            region: region,
            namespace: namespace,
            locale: locale,
        }
    }
}

#[derive(Debug)]
pub struct BlizzardApiRS {
    settings: Settings,
}

impl BlizzardApiRS {
    pub fn new(region: String, namespace: String, locale: String) -> BlizzardApiRS {
        BlizzardApiRS {
            settings: Settings::new(region, namespace, locale),
        }
    }

    pub async fn get_token(
        &mut self,
        client: &String,
        secret: &String,
    ) -> Result<OAuthToken, String> {
        match BattleNetApi::get_token(client, secret, &self.settings).await {
            Ok(token) => Ok(token),
            Err(msg) => Err(msg.to_string()),
        }
    }

    pub async fn get_wow_character_profile(
        &self,
        token: &OAuthToken,
        server: &String,
        name: &String,
    ) -> Result<CharacterProfile, String> {
        match WowApi::character_profile(token, server, name, &self.settings).await {
            Ok(resp) => Ok(resp),
            Err(msg) => Err(msg.to_string()),
        }
    }

    pub async fn get_wow_character_media(
        &self,
        token: &OAuthToken,
        server: &String,
        name: &String,
    ) -> Result<CharacterMedia, String> {
        match WowApi::character_media(token, server, name, &self.settings).await {
            Ok(resp) => Ok(resp),
            Err(msg) => Err(msg.to_string()),
        }
    }

    pub async fn get_wow_character_statistics(
        &self,
        token: &OAuthToken,
        server: &String,
        name: &String,
    ) -> Result<CharacterStatistics, String> {
        match WowApi::character_statistics(token, server, name, &self.settings).await {
            Ok(resp) => Ok(resp),
            Err(msg) => Err(msg.to_string()),
        }
    }

    pub async fn get_wow_character_equipment(
        &self,
        token: &OAuthToken,
        server: &String,
        name: &String,
    ) -> Result<CharacterEquipment, String> {
        match WowApi::character_equipment(token, server, name, &self.settings).await {
            Ok(resp) => Ok(resp),
            Err(msg) => Err(msg.to_string()),
        }
    }
}
