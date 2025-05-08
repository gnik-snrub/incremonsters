use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::models::Monster;
use crate::monsters::GrowthRates;
//use crate::traits::celestials::CelestialTrait;
//use crate::traits::TraitTrait;

use super::MonsterType;

#[derive(Clone, Copy, Debug)]
pub enum AbyssbornType {
    Glowdrifter,
    Tentaclaw,
    Abyssmaw,
    Veilshroud,
}

impl MonsterType for AbyssbornType {
    fn generate(&self) -> Monster {
        match self {
            AbyssbornType::Glowdrifter => {
                let mut temp = Monster::new("Abyssborn", "Glowdrifter", 200, 10, 10, 10);
                //temp.add_trait(CelestialTrait::Radiantheart.create());
                temp
            },
            AbyssbornType::Tentaclaw => {
                let mut temp = Monster::new("Abyssborn", "Tentaclaw", 100, 22, 10, 18);
                //temp.add_trait(CelestialTrait::Aetherwing.create());
                temp
            },
            AbyssbornType::Abyssmaw => {
                let mut temp = Monster::new("Abyssborn", "Abyssmaw", 100, 17, 20, 13);
                //temp.add_trait(CelestialTrait::Aurenguard.create());
                temp
            },
            AbyssbornType::Veilshroud => {
                let mut temp = Monster::new("Abyssborn", "Veilshroud", 125, 15, 15, 15);
                //temp.add_trait(CelestialTrait::Divinarch.create());
                temp
            },
        }
    }
    fn random() -> Self {
        let options = [AbyssbornType::Glowdrifter, AbyssbornType::Tentaclaw, AbyssbornType::Abyssmaw, AbyssbornType::Veilshroud];
        *options.choose(&mut thread_rng()).unwrap()
    }
}

pub fn get_growth_rate(type_: AbyssbornType) -> GrowthRates {
    match type_ {
        AbyssbornType::Glowdrifter => GLOWDRIFTER_GROWTH_RATE,
        AbyssbornType::Tentaclaw => TENTACLAW_GROWTH_RATE,
        AbyssbornType::Abyssmaw => ABYSSMAW_GROWTH_RATE,
        AbyssbornType::Veilshroud => VEILSHROUD_GROWTH_RATE,
    }
}

pub const GLOWDRIFTER_GROWTH_RATE: GrowthRates = GrowthRates {
    hp: 1.11,
    atk: 1.01,
    def: 1.01,
    spd: 1.01,
};

pub const TENTACLAW_GROWTH_RATE: GrowthRates = GrowthRates {
    hp: 1.01,
    atk: 1.06,
    def: 1.01,
    spd: 1.04,
};

pub const ABYSSMAW_GROWTH_RATE: GrowthRates = GrowthRates {
    hp: 1.01,
    atk: 1.045,
    def: 1.06,
    spd: 1.025,
};

pub const VEILSHROUD_GROWTH_RATE: GrowthRates = GrowthRates {
    hp: 1.035,
    atk: 1.035,
    def: 1.035,
    spd: 1.035,
};
