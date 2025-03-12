use serde::{Deserialize, Serialize};
use celestials::CelestialTrait;
use stonekin::StonekinTrait;

use crate::models::{CallbackFn, Trait};

pub mod stonekin;
pub mod celestials;

#[derive(PartialEq, PartialOrd, Deserialize, Serialize, Clone, Debug)]
pub enum MonsterTrait {
    Stonekin(StonekinTrait),
    Celestial(CelestialTrait),
}

pub trait TraitTrait {
    fn create(self) -> Trait;
    fn get(&self) -> CallbackFn;
}

pub fn get_callback(trait_: MonsterTrait) -> CallbackFn {
    match trait_ {
        MonsterTrait::Stonekin(StonekinTrait::Slateblade) => stonekin::cliffs_edge,
        MonsterTrait::Stonekin(StonekinTrait::Pebblebound) => stonekin::quaking_dodge,
        MonsterTrait::Stonekin(StonekinTrait::Bolderfist) => stonekin::shared_earth_armor,
        MonsterTrait::Stonekin(StonekinTrait::Mountainheart) => stonekin::titanic_retaliation,

        MonsterTrait::Celestial(CelestialTrait::Radiantheart) => celestials::ward_of_renewal,
        MonsterTrait::Celestial(CelestialTrait::Aetherwing) => celestials::ward_of_vengeance,
        MonsterTrait::Celestial(CelestialTrait::HaloSentinel) => celestials::ward_of_aegis,
        MonsterTrait::Celestial(CelestialTrait::Divinarch) => celestials::ward_of_sanctification,
    }
}
