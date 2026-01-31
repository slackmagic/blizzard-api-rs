use serde::{Deserialize, Serialize};
use crate::wow::retail::CharacterRef;
//
// -------------------- Commun --------------------
//

#[derive(Debug, Serialize, Deserialize)]
pub struct Key {
    pub href: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NamedType {
    #[serde(rename = "type")]
    pub type_: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NamedResource {
    pub key: Key,
    pub name: String,
    pub id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Display {
    pub display_string: String,
    pub color: Option<Color>,
}

//
// -------------------- Liens --------------------
//

#[derive(Debug, Serialize, Deserialize)]
pub struct Links {
    #[serde(rename = "self")]
    pub self_: Key,
}

//
// -------------------- Item --------------------
//

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemRef {
    pub key: Key,
    pub id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Media {
    pub key: Key,
    pub id: u64,
}

//
// -------------------- Stats --------------------
//

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

//
// -------------------- Armor / Durability --------------------
//

#[derive(Debug, Serialize, Deserialize)]
pub struct Armor {
    pub value: u64,
    pub display: Display,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Durability {
    pub value: u64,
    pub display_string: String,
}

//
// -------------------- Sell price --------------------
//

#[derive(Debug, Serialize, Deserialize)]
pub struct SellPrice {
    pub value: u64,
    pub display_strings: PriceDisplay,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PriceDisplay {
    pub header: String,
    pub gold: String,
    pub silver: String,
    pub copper: String,
}

//
// -------------------- Requirements --------------------
//

#[derive(Debug, Serialize, Deserialize)]
pub struct Requirements {
    pub level: Option<LevelRequirement>,
    pub playable_classes: Option<PlayableClasses>,
    pub display_string: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LevelRequirement {
    pub value: u64,
    pub display_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayableClasses {
    pub links: Vec<NamedResource>,
    pub display_string: String,
}

//
// -------------------- Sockets --------------------
//

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

//
// -------------------- Spells --------------------
//

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemSpell {
    pub spell: NamedResource,
    pub description: String,
}

//
// -------------------- Transmog --------------------
//

#[derive(Debug, Serialize, Deserialize)]
pub struct Transmog {
    pub item: NamedResource,
    pub display_string: String,
    pub item_modified_appearance_id: u64,
}

//
// -------------------- Item Set --------------------
//

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemSet {
    pub item_set: NamedResource,
    pub items: Vec<SetItem>,
    pub effects: Vec<SetEffect>,
    pub display_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetItem {
    pub item: NamedResource,

    #[serde(default)]
    pub is_equipped: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetEffect {
    pub display_string: String,
    pub required_count: u64,

    #[serde(default)]
    pub is_active: Option<bool>,
}

//
// -------------------- Item Level --------------------
//

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemLevel {
    pub value: u64,
    pub display_string: String,
}

//
// -------------------- Equipped Item --------------------
//

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

//
// -------------------- Root --------------------
//

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterEquipment {
    pub _links: Links,
    pub character: CharacterRef,
    pub equipped_items: Vec<EquippedItem>,
}
