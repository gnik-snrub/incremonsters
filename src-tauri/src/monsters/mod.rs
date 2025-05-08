use abyssborn::{AbyssbornType, ABYSSMAW_GROWTH_RATE, GLOWDRIFTER_GROWTH_RATE, TENTACLAW_GROWTH_RATE, VEILSHROUD_GROWTH_RATE};
use celestials::{CelestialType, AETHERWING_GROWTH_RATE, AURENGUARD_GROWTH_RATE, DIVINARCH_GROWTH_RATE, RADIANTHEART_GROWTH_RATE};
use stonekin::{ StonekinType, PEBBLEBOUND_GROWTH_RATE, SLATEBLADE_GROWTH_RATE, BOLDERFIST_GROWTH_RATE, MOUNTAINHEART_GROWTH_RATE};

use crate::{math::rewards::{GrowthBoosts, GrowthModifier}, models::Monster};

pub mod stonekin;
pub mod celestials;
pub mod abyssborn;

#[derive(Clone)]
pub enum MonsterFamily {
    Stonekin(Option<StonekinType>),
    Celestial(Option<CelestialType>),
    Abyssborn(Option<AbyssbornType>)
}

pub trait MonsterType {
    fn generate(&self) -> Monster;
    fn random() -> Self;
}

#[derive(Debug)]
pub struct GrowthRates {
    pub hp: f32,
    pub atk: f32,
    pub def: f32,
    pub spd: f32,
}

const MISSING_GROWTH_RATE: GrowthRates = GrowthRates {
    hp: 1.0,
    atk: 1.0,
    def: 1.0,
    spd: 1.0,
};

pub fn level_up(monster: &mut Monster, modifiers: &GrowthBoosts) {
    let growth_rates: GrowthRates = find_growth_rate(monster.clone());
    monster.lvl += 1;
    monster.hp = get_new_stat(monster.hp, growth_rates.hp, &modifiers.0);
    monster.atk = get_new_stat(monster.atk, growth_rates.atk, &modifiers.1);
    monster.def = get_new_stat(monster.def, growth_rates.def, &modifiers.2);
    monster.spd = get_new_stat(monster.spd, growth_rates.spd, &modifiers.3);
}

fn get_new_stat(original_stat: i32, growth_rate: f32, modifiers: &Vec<GrowthModifier>) -> i32 {
    let mut new_stat = original_stat as f32;
    let mut final_growth_rate = growth_rate;

    for modifier in modifiers {
        if modifier.operation == "add" {
            final_growth_rate += modifier.magnitude;
        } else if modifier.operation == "mult" {
            final_growth_rate *= modifier.magnitude;
        }
    }
    new_stat = (new_stat * final_growth_rate).ceil();
    new_stat as i32
}

fn find_growth_rate(monster: Monster) -> GrowthRates {
    let rate = match monster.creature_type.as_str() {
        "Pebblebound" => PEBBLEBOUND_GROWTH_RATE,
        "Slateblade" => SLATEBLADE_GROWTH_RATE,
        "Bolderfist" => BOLDERFIST_GROWTH_RATE,
        "Mountainheart" => MOUNTAINHEART_GROWTH_RATE,

        "Radiantheart" => RADIANTHEART_GROWTH_RATE,
        "Aetherwing" => AETHERWING_GROWTH_RATE,
        "Aurenguard" => AURENGUARD_GROWTH_RATE,
        "Divinarch" => DIVINARCH_GROWTH_RATE,

        "Glowdrifter" => GLOWDRIFTER_GROWTH_RATE,
        "Tentaclaw" => TENTACLAW_GROWTH_RATE,
        "Abyssmaw" => ABYSSMAW_GROWTH_RATE,
        "Veilshroud" => VEILSHROUD_GROWTH_RATE,

        _ => MISSING_GROWTH_RATE,
    };
    rate
}

pub fn generate_monster(family: MonsterFamily) -> Monster {
    match family {
        MonsterFamily::Stonekin(None) => StonekinType::random().generate(),
        MonsterFamily::Stonekin(Some(monster)) => monster.generate(),
        MonsterFamily::Celestial(None) => CelestialType::random().generate(),
        MonsterFamily::Celestial(Some(monster)) => monster.generate(),
        MonsterFamily::Abyssborn(None) => AbyssbornType::random().generate(),
        MonsterFamily::Abyssborn(Some(monster)) => monster.generate(),
    }
}
