#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterMedia {
    assets: Option<Vec<Media>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Media {
    key: String,
    value: String,
}
