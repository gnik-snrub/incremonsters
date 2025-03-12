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
    HaloSentinel,
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
            CelestialType::HaloSentinel => {
                let mut temp = Monster::new("Celestial", "Halo Sentinel", 100, 17, 20, 13);
                temp.add_trait(CelestialTrait::HaloSentinel.create());
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
        let options = [CelestialType::Radiantheart, CelestialType::Aetherwing, CelestialType::HaloSentinel, CelestialType::Divinarch];
        *options.choose(&mut thread_rng()).unwrap()
    }
}
