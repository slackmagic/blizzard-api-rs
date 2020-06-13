#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterStatistics {
    health: u64,
    power: u64,
    agility: Agility,
    intellect: Intellect,
    stamina: Stamina,
    strength: Strength,
    armor: Armor,
    speed: Speed,
    melee_haste: MeleeHaste,
    melee_crit: MeleeCrit,
    spell_haste: SpellHaste,
    spell_crit: SpellCrit,
    ranged_haste: RangedHaste,
    ranged_crit: RangedCrit,
    mastery: Mastery,
    lifesteal: Lifesteal,
    dodge: Dodge,
    parry: Parry,
    block: Block,
    avoidance: Avoidance,
    bonus_armor: u64,
    versatility_damage_done_bonus: f64,
    versatility_healing_done_bonus: f64,
    versatility_damage_taken_bonus: f64,
    attack_power: u64,
    main_hand_damage_min: f64,
    main_hand_damage_max: f64,
    main_hand_speed: f64,
    main_hand_dps: f64,
    off_hand_damage_min: f64,
    off_hand_damage_max: f64,
    off_hand_speed: f64,
    off_hand_dps: f64,
    spell_power: u64,
    spell_penetration: u64,
    mana_regen: f64,
    mana_regen_combat: f64,
    corruption: Option<Corruption>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Agility {
    base: u64,
    effective: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Strength {
    base: u64,
    effective: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Intellect {
    base: u64,
    effective: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stamina {
    base: u64,
    effective: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Armor {
    base: u64,
    effective: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Speed {
    rating: u64,
    rating_bonus: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeleeCrit {
    rating: u64,
    rating_bonus: f64,
    value: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeleeHaste {
    rating: u64,
    rating_bonus: f64,
    value: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Mastery {
    rating: u64,
    rating_bonus: f64,
    value: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Lifesteal {
    rating: u64,
    rating_bonus: f64,
    value: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpellCrit {
    rating: u64,
    rating_bonus: f64,
    value: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpellHaste {
    rating: u64,
    rating_bonus: f64,
    value: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RangedCrit {
    rating: u64,
    rating_bonus: f64,
    value: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RangedHaste {
    rating: u64,
    rating_bonus: f64,
    value: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Dodge {
    rating: u64,
    rating_bonus: f64,
    value: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Parry {
    rating: u64,
    rating_bonus: f64,
    value: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    rating: u64,
    rating_bonus: f64,
    value: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Avoidance {
    rating: u64,
    rating_bonus: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Corruption {
    corruption: f64,
    corruption_resistance: f64,
    effective_corruption: f64,
}
