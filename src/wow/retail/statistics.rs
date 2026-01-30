use serde::{Deserialize, Serialize};
use crate::wow::commons::*;
use super::character::Character;

/// Statistiques de personnage (V2)
#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterStatistics {
    pub _links: Links,

    pub health: u64,
    pub power: u64,

    pub power_type: NamedResource,

    pub speed: RatingStat,

    pub strength: BaseEffective,
    pub agility: BaseEffective,
    pub intellect: BaseEffective,
    pub stamina: BaseEffective,

    pub melee_crit: RatingValueStat,
    pub melee_haste: RatingValueStat,
    pub mastery: RatingValueStat,

    pub bonus_armor: u64,

    pub lifesteal: RatingValueStat,

    pub versatility: f64,
    pub versatility_damage_done_bonus: f64,
    pub versatility_healing_done_bonus: f64,
    pub versatility_damage_taken_bonus: f64,

    pub avoidance: RatingStat,

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

    pub spell_crit: RatingValueStat,

    pub mana_regen: f64,
    pub mana_regen_combat: f64,

    pub armor: BaseEffective,

    pub dodge: RatingValueStat,
    pub parry: RatingValueStat,
    pub block: RatingValueStat,

    pub ranged_crit: RatingValueStat,
    pub ranged_haste: RatingValueStat,
    pub spell_haste: RatingValueStat,

    pub character: Character,
}
