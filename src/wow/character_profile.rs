#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterProfile {
    pub id: u64,
    pub name: String,
    pub character_class: CharacterClass,
    pub faction: Faction,
    pub active_spec: ActiveSpec,
    pub race: Race,
    pub realm: Realm,
    pub guild: Option<Guild>,
    pub active_title: Title,
    pub level: u64,
    pub experience: u64,
    pub achievement_points: u64,
    pub average_item_level: u64,
    pub equipped_item_level: u64,
    pub last_login_timestamp: u64,
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
