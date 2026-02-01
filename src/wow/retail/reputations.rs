use serde::{Deserialize, Serialize};
use crate::wow::commons::*;
use super::character::Realm;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CharacterReputations {
    #[serde(rename = "_links")]
    pub links: Links,
    pub character: CharacterReference,
    pub reputations: Vec<Reputation>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CharacterReference {
    pub key: Key,
    pub name: String,
    pub id: u64,
    pub realm: Realm,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Reputation {
    pub faction: Faction,
    pub standing: Standing,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Faction {
    pub key: Key,
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Standing {
    pub raw: i32,
    pub value: i32,
    pub max: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<u8>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renown_level: Option<u16>,
}
