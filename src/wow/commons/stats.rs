use serde::{Deserialize, Serialize};

/// Stats avec valeur de base et effective
#[derive(Debug, Serialize, Deserialize)]
pub struct BaseEffective {
    pub base: u64,
    pub effective: u64,
}

/// Stats avec rating (bonus et normalisé)
#[derive(Debug, Serialize, Deserialize)]
pub struct RatingStat {
    pub rating_bonus: f64,
    pub rating_normalized: u64,
}

/// Stats avec rating et valeur
#[derive(Debug, Serialize, Deserialize)]
pub struct RatingValueStat {
    pub rating_bonus: f64,
    pub value: f64,
    pub rating_normalized: u64,
}
