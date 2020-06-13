#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterProfile {
    id: u64,
    name: String,
    character_class: CharacterClass,
    faction: Faction,
    active_spec: ActiveSpec,
    race: Race,
    realm: Realm,
    guild: Guild,
    active_title: Title,
    level: u64,
    experience: u64,
    achievement_points: u64,
    average_item_level: u64,
    equipped_item_level: u64,
    last_login_timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActiveSpec {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Faction {
    #[serde(alias = "type")]
    type_faction: String,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterClass {
    id: u64,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Race {
    id: u64,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Realm {
    id: u64,
    name: String,
    slug: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Guild {
    id: u64,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Title {
    id: u64,
    name: String,
    display_string: String,
}
