#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CharacterMedia {
    pub assets: Option<Vec<Media>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Media {
    key: String,
    value: String,
}
