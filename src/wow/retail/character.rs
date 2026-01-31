use serde::{Deserialize, Serialize};
use crate::wow::commons::*;

/// Royaume (realm)
#[derive(Debug, Serialize, Deserialize)]
pub struct Realm {
    pub key: BlizzardKey,
    pub name: String,
    pub id: u64,
    pub slug: String,
}

/// Personnage
#[derive(Debug, Serialize, Deserialize)]
pub struct Character {
    pub key: BlizzardKey,
    pub name: String,
    pub id: u64,
    pub realm: Realm,
}

/// Référence à un personnage
#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterRef {
    pub key: Key,
    pub name: String,
    pub id: u64,
    pub realm: Realm,
}

/// Media du personnage
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CharacterMedia {
    pub assets: Option<Vec<Media>>,
}

/// Guilde
#[derive(Debug, Serialize, Deserialize)]
pub struct Guild {
    pub key: BlizzardKey,
    pub name: String,
    pub id: u64,
    pub realm: Realm,
    pub faction: NamedType,
}

/// Progression de covenant
#[derive(Debug, Serialize, Deserialize)]
pub struct CovenantProgress {
    pub chosen_covenant: NamedResource,
    pub renown_level: u64,
    pub soulbinds: HrefOnly,
}

/// Titre actif
#[derive(Debug, Serialize, Deserialize)]
pub struct ActiveTitle {
    pub key: BlizzardKey,
    pub name: String,
    pub id: u64,
    pub display_string: String,
}

/// Profil de personnage (V2)
#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterProfile {
    pub _links: Links,

    pub id: u64,
    pub name: String,

    pub gender: NamedType,
    pub faction: NamedType,

    pub race: NamedResource,

    #[serde(rename = "character_class")]
    pub class_: NamedResource,

    pub active_spec: NamedResource,

    pub realm: Realm,
    pub guild: Option<Guild>,

    pub level: u64,
    pub experience: u64,
    pub achievement_points: u64,

    pub achievements: HrefOnly,
    pub titles: HrefOnly,
    pub pvp_summary: HrefOnly,
    pub encounters: HrefOnly,
    pub media: HrefOnly,

    pub last_login_timestamp: u64,

    pub average_item_level: u64,
    pub equipped_item_level: u64,

    pub specializations: HrefOnly,
    pub statistics: HrefOnly,
    pub mythic_keystone_profile: HrefOnly,
    pub equipment: HrefOnly,
    pub appearance: HrefOnly,
    pub collections: HrefOnly,

    pub active_title: Option<ActiveTitle>,

    pub reputations: HrefOnly,
    pub quests: HrefOnly,
    pub achievements_statistics: HrefOnly,
    pub professions: HrefOnly,

    pub covenant_progress: Option<CovenantProgress>,

    pub is_remix: bool,

    #[serde(default)]
    pub houses: Vec<HrefOnly>,

    pub name_search: String,
}
