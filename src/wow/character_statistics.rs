#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CharacterStatistics {
    pub health: u64,
    pub power: u64,
    pub agility: Agility,
    pub intellect: Intellect,
    pub stamina: Stamina,
    pub strength: Strength,
    pub armor: Armor,
    pub speed: Speed,
    pub melee_haste: MeleeHaste,
    pub melee_crit: MeleeCrit,
    pub spell_haste: SpellHaste,
    pub spell_crit: SpellCrit,
    pub ranged_haste: RangedHaste,
    pub ranged_crit: RangedCrit,
    pub mastery: Mastery,
    pub lifesteal: Lifesteal,
    pub dodge: Dodge,
    pub parry: Parry,
    pub block: Block,
    pub avoidance: Avoidance,
    pub bonus_armor: u64,
    pub versatility_damage_done_bonus: f64,
    pub versatility_healing_done_bonus: f64,
    pub versatility_damage_taken_bonus: f64,
    pub attack_power: u64,
    pub main_hand_damage_min: f64,
    pub main_hand_damage_max: f64,
    pub main_hand_speed: f64,
    pub main_hand_dps: f64,
    pub off_hand_damage_min: f64,
    pub off_hand_damage_max: f64,
    pub off_hand_speed: f64,
    pub off_hand_dps: f64,
    pub spell_power: u64,
    pub spell_penetration: u64,
    pub mana_regen: f64,
    pub mana_regen_combat: f64,
    pub corruption: Option<Corruption>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Agility {
    base: u64,
    effective: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Strength {
    base: u64,
    effective: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Intellect {
    base: u64,
    effective: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Stamina {
    base: u64,
    effective: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Armor {
    base: u64,
    effective: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Speed {
    rating: u64,
    rating_bonus: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MeleeCrit {
    rating: u64,
    rating_bonus: f64,
    value: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MeleeHaste {
    rating: u64,
    rating_bonus: f64,
    value: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Mastery {
    rating: u64,
    rating_bonus: f64,
    value: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Lifesteal {
    rating: u64,
    rating_bonus: f64,
    value: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SpellCrit {
    rating: u64,
    rating_bonus: f64,
    value: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SpellHaste {
    rating: u64,
    rating_bonus: f64,
    value: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RangedCrit {
    rating: u64,
    rating_bonus: f64,
    value: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RangedHaste {
    rating: u64,
    rating_bonus: f64,
    value: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Dodge {
    rating: u64,
    rating_bonus: f64,
    value: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Parry {
    rating: u64,
    rating_bonus: f64,
    value: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Block {
    rating: u64,
    rating_bonus: f64,
    value: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Avoidance {
    rating: u64,
    rating_bonus: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Corruption {
    corruption: f64,
    corruption_resistance: f64,
    effective_corruption: f64,
}
