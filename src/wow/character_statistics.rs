#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterStatistics {
    health: usize,
    power: usize,
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
    bonus_armor: usize,
    versatility_damage_done_bonus: f64,
    versatility_healing_done_bonus: f64,
    versatility_damage_taken_bonus: f64,
    attack_power: usize,
    main_hand_damage_min: f64,
    main_hand_damage_max: f64,
    main_hand_speed: f64,
    main_hand_dps: f64,
    off_hand_damage_min: f64,
    off_hand_damage_max: f64,
    off_hand_speed: f64,
    off_hand_dps: f64,
    spell_power: usize,
    spell_penetration: usize,
    mana_regen: f64,
    mana_regen_combat: f64,
    corruption: Option<Corruption>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Agility {
    base: usize,
    effective: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Strength {
    base: usize,
    effective: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Intellect {
    base: usize,
    effective: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stamina {
    base: usize,
    effective: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Armor {
    base: usize,
    effective: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Speed {
    rating: usize,
    rating_bonus: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeleeCrit {
    rating: usize,
    rating_bonus: f64,
    value: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeleeHaste {
    rating: usize,
    rating_bonus: f64,
    value: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Mastery {
    rating: usize,
    rating_bonus: f64,
    value: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Lifesteal {
    rating: usize,
    rating_bonus: f64,
    value: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpellCrit {
    rating: usize,
    rating_bonus: f64,
    value: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpellHaste {
    rating: usize,
    rating_bonus: f64,
    value: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RangedCrit {
    rating: usize,
    rating_bonus: f64,
    value: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RangedHaste {
    rating: usize,
    rating_bonus: f64,
    value: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Dodge {
    rating: usize,
    rating_bonus: f64,
    value: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Parry {
    rating: usize,
    rating_bonus: f64,
    value: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    rating: usize,
    rating_bonus: f64,
    value: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Avoidance {
    rating: usize,
    rating_bonus: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Corruption {
    corruption: f64,
    corruption_resistance: f64,
    effective_corruption: f64,
}
