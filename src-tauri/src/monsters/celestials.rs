use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::models::Monster;
use crate::monsters::GrowthRates;
use crate::traits::celestials::CelestialTrait;
use crate::traits::TraitTrait;

use super::MonsterType;

#[derive(Clone, Copy, Debug)]
pub enum CelestialType {
    Radiantheart,
    Aetherwing,
    Aurenguard,
    Divinarch,
}

impl MonsterType for CelestialType {
    fn generate(&self) -> Monster {
        match self {
            CelestialType::Radiantheart => {
                let mut temp = Monster::new("Celestial", "Radiantheart", 200, 10, 10, 10);
                temp.add_trait(CelestialTrait::Radiantheart.create());
                temp
            },
            CelestialType::Aetherwing => {
                let mut temp = Monster::new("Celestial", "Aetherwing", 100, 22, 10, 18);
                temp.add_trait(CelestialTrait::Aetherwing.create());
                temp
            },
            CelestialType::Aurenguard => {
                let mut temp = Monster::new("Celestial", "Aurenguard", 100, 17, 20, 13);
                temp.add_trait(CelestialTrait::Aurenguard.create());
                temp
            },
            CelestialType::Divinarch => {
                let mut temp = Monster::new("Celestial", "Divinarch", 125, 15, 15, 15);
                temp.add_trait(CelestialTrait::Divinarch.create());
                temp
            },
        }
    }
    fn random() -> Self {
        let options = [CelestialType::Radiantheart, CelestialType::Aetherwing, CelestialType::Aurenguard, CelestialType::Divinarch];
        *options.choose(&mut thread_rng()).unwrap()
    }
}

pub fn get_growth_rate(type_: CelestialType) -> GrowthRates {
    match type_ {
        CelestialType::Radiantheart => RADIANTHEART_GROWTH_RATE,
        CelestialType::Aetherwing => AETHERWING_GROWTH_RATE,
        CelestialType::Aurenguard => AURENGUARD_GROWTH_RATE,
        CelestialType::Divinarch => DIVINARCH_GROWTH_RATE,
    }
}

pub const RADIANTHEART_GROWTH_RATE: GrowthRates = GrowthRates {
    hp: 1.11,
    atk: 1.01,
    def: 1.01,
    spd: 1.01,
};

pub const AETHERWING_GROWTH_RATE: GrowthRates = GrowthRates {
    hp: 1.01,
    atk: 1.06,
    def: 1.01,
    spd: 1.04,
};

pub const AURENGUARD_GROWTH_RATE: GrowthRates = GrowthRates {
    hp: 1.01,
    atk: 1.045,
    def: 1.06,
    spd: 1.025,
};

pub const DIVINARCH_GROWTH_RATE: GrowthRates = GrowthRates {
    hp: 1.035,
    atk: 1.035,
    def: 1.035,
    spd: 1.035,
};
