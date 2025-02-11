use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::models::Monster;
use crate::monsters::GrowthRates;

use super::MonsterType;

#[derive(Clone, Copy)]
pub enum StonekinType {
    Slateblade,
    Pebblebound,
    Bolderfist,
    Mountainheart,
}

impl MonsterType for StonekinType {
    fn generate(&self) -> Monster {
        match self {
            StonekinType::Slateblade => Monster::new("Stonekin", "Slateblade", 100, 17, 15, 18),
            StonekinType::Pebblebound => Monster::new("Stonekin", "Pebblebound", 100, 12, 18, 20),
            StonekinType::Bolderfist => Monster::new("Stonekin", "Bolderfist", 100, 15, 25, 10),
            StonekinType::Mountainheart => Monster::new("Stonekin", "Mountainheart", 110, 20, 20, 8),
        }
    }
    fn random() -> Self {
        let options = [StonekinType::Slateblade, StonekinType::Pebblebound, StonekinType::Bolderfist, StonekinType::Mountainheart];
        *options.choose(&mut thread_rng()).unwrap()
    }
}

pub fn get_growth_rate(type_: StonekinType) -> GrowthRates {
    match type_ {
        StonekinType::Slateblade => SLATEBLADE_GROWTH_RATE,
        StonekinType::Pebblebound => PEBBLEBOUND_GROWTH_RATE,
        StonekinType::Bolderfist => BOLDERFIST_GROWTH_RATE,
        StonekinType::Mountainheart => MOUNTAINHEART_GROWTH_RATE,
    }
}

pub const SLATEBLADE_GROWTH_RATE: GrowthRates = GrowthRates {
    hp: 1.05,
    atk: 1.045,
    def: 1.035,
    spd: 1.05,
};

pub const PEBBLEBOUND_GROWTH_RATE: GrowthRates = GrowthRates {
    hp: 1.05,
    atk: 1.02,
    def: 1.05,
    spd: 1.06,
};

pub const BOLDERFIST_GROWTH_RATE: GrowthRates = GrowthRates {
    hp: 1.05,
    atk: 1.035,
    def: 1.085,
    spd: 1.01,
};

pub const MOUNTAINHEART_GROWTH_RATE: GrowthRates = GrowthRates {
    hp: 1.05,
    atk: 1.06,
    def: 1.06,
    spd: 1.01,
};
