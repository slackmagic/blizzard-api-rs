#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterMedia {
    assets: Vec<Media>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Media {
    key: String,
    url: String,
}
