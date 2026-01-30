use serde::{Deserialize, Serialize};
use crate::wow::commons::*;
use super::character::CharacterRef;

/// Référence à un item
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemRef {
    pub key: Key,
    pub id: u64,
}

/// Stat d'item
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemStat {
    #[serde(rename = "type")]
    pub stat_type: NamedType,
    pub value: i64,

    #[serde(default)]
    pub is_equip_bonus: bool,

    #[serde(default)]
    pub is_negated: bool,

    pub display: Display,
}

/// Armure
#[derive(Debug, Serialize, Deserialize)]
pub struct Armor {
    pub value: u64,
    pub display: Display,
}

/// Durabilité
#[derive(Debug, Serialize, Deserialize)]
pub struct Durability {
    pub value: u64,
    pub display_string: String,
}

/// Prix de vente
#[derive(Debug, Serialize, Deserialize)]
pub struct SellPrice {
    pub value: u64,
    pub display_strings: PriceDisplay,
}

/// Affichage du prix
#[derive(Debug, Serialize, Deserialize)]
pub struct PriceDisplay {
    pub header: String,
    pub gold: String,
    pub silver: String,
    pub copper: String,
}

/// Prérequis d'équipement
#[derive(Debug, Serialize, Deserialize)]
pub struct Requirements {
    pub level: Option<LevelRequirement>,
    pub playable_classes: Option<PlayableClasses>,
    pub display_string: Option<String>,
}

/// Prérequis de niveau
#[derive(Debug, Serialize, Deserialize)]
pub struct LevelRequirement {
    pub value: u64,
    pub display_string: String,
}

/// Classes jouables
#[derive(Debug, Serialize, Deserialize)]
pub struct PlayableClasses {
    pub links: Vec<NamedResource>,
    pub display_string: String,
}

/// Socket d'item
#[derive(Debug, Serialize, Deserialize)]
pub struct Socket {
    pub socket_type: NamedType,

    #[serde(default)]
    pub item: Option<NamedResource>,

    #[serde(default)]
    pub context: Option<u64>,

    #[serde(default)]
    pub display_string: Option<String>,

    #[serde(default)]
    pub media: Option<Media>,
}

/// Sort d'item
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemSpell {
    pub spell: NamedResource,
    pub description: String,
}

/// Transmog
#[derive(Debug, Serialize, Deserialize)]
pub struct Transmog {
    pub item: NamedResource,
    pub display_string: String,
    pub item_modified_appearance_id: u64,
}

/// Set d'items
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemSet {
    pub item_set: NamedResource,
    pub items: Vec<SetItem>,
    pub effects: Vec<SetEffect>,
    pub display_string: String,
}

/// Item de set
#[derive(Debug, Serialize, Deserialize)]
pub struct SetItem {
    pub item: NamedResource,
    pub is_equipped: bool,
}

/// Effet de set
#[derive(Debug, Serialize, Deserialize)]
pub struct SetEffect {
    pub display_string: String,
    pub required_count: u64,
    pub is_active: bool,
}

/// Niveau d'item
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemLevel {
    pub value: u64,
    pub display_string: String,
}

/// Item équipé
#[derive(Debug, Serialize, Deserialize)]
pub struct EquippedItem {
    pub item: ItemRef,

    #[serde(default)]
    pub sockets: Vec<Socket>,

    pub slot: NamedType,
    pub quantity: u64,

    #[serde(default)]
    pub context: Option<u64>,

    #[serde(default)]
    pub bonus_list: Vec<u64>,

    pub quality: NamedType,
    pub name: String,

    #[serde(default)]
    pub modified_appearance_id: Option<u64>,

    pub media: Media,

    pub item_class: NamedResource,
    pub item_subclass: NamedResource,
    pub inventory_type: NamedType,
    pub binding: NamedType,

    #[serde(default)]
    pub unique_equipped: Option<String>,

    #[serde(default)]
    pub armor: Option<Armor>,

    #[serde(default)]
    pub stats: Vec<ItemStat>,

    #[serde(default)]
    pub spells: Vec<ItemSpell>,

    #[serde(default)]
    pub sell_price: Option<SellPrice>,

    #[serde(default)]
    pub requirements: Option<Requirements>,

    pub level: ItemLevel,

    #[serde(default)]
    pub transmog: Option<Transmog>,

    #[serde(default)]
    pub durability: Option<Durability>,

    #[serde(default)]
    pub set: Option<ItemSet>,

    #[serde(default)]
    pub is_subclass_hidden: bool,

    #[serde(default)]
    pub name_description: Option<Display>,
}

/// Équipement de personnage
#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterEquipment {
    pub _links: Links,
    pub character: CharacterRef,
    pub equipped_items: Vec<EquippedItem>,
}
