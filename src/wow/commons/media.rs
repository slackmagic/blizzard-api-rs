use serde::{Deserialize, Serialize};

/// Media générique (clé/valeur)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Media {
    pub key: String,
    pub value: String,
}
