#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterProfile {
    id: usize,
    name: String,
    faction: Faction,
    active_spec: ActiveSpec,
    race: Race,
    realm: Realm,
    guild: Guild,
    level: usize,
    experience: usize,
    achievement_points: usize,
    average_item_level: usize,
    equipped_item_level: usize,
    last_login_timestamp: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActiveSpec {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Faction {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterClass {
    id: usize,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Race {
    id: usize,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Realm {
    id: usize,
    name: String,
    slug: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Guild {
    id: usize,
    name: String,
}
