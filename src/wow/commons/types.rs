use serde::{Deserialize, Serialize};

/// Lien générique Blizzard
#[derive(Debug, Serialize, Deserialize)]
pub struct BlizzardLink {
    pub href: String,
}

/// Clé générique Blizzard
#[derive(Debug, Serialize, Deserialize)]
pub struct BlizzardKey {
    pub href: String,
}

/// Ressource nommée avec clé, nom et id
#[derive(Debug, Serialize, Deserialize)]
pub struct NamedResource {
    pub key: BlizzardKey,
    pub name: String,
    pub id: u64,
}

/// Type nommé générique
#[derive(Debug, Serialize, Deserialize)]
pub struct NamedType {
    #[serde(rename = "type")]
    pub type_: String,
    pub name: String,
}

/// Structure de liens (avec self)
#[derive(Debug, Serialize, Deserialize)]
pub struct Links {
    #[serde(rename = "self")]
    pub self_: BlizzardLink,
}

/// Référence href simple
#[derive(Debug, Serialize, Deserialize)]
pub struct HrefOnly {
    pub href: String,
}

/// Clé simple
#[derive(Debug, Serialize, Deserialize)]
pub struct Key {
    pub href: String,
}

/// Couleur RGBA
#[derive(Debug, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: f32,
}

/// Affichage avec texte et couleur optionnelle
#[derive(Debug, Serialize, Deserialize)]
pub struct Display {
    pub display_string: String,
    pub color: Option<Color>,
}
