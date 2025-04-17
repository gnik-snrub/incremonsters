use serde::{Deserialize, Serialize};
use celestials::CelestialTrait;
use stonekin::StonekinTrait;

use crate::{Monster, models::{CallbackFn, Trait}};

pub mod stonekin;
use stonekin::{cliffs_edge, quaking_dodge, shared_earth_armor, titanic_retaliation};
pub mod celestials;
use celestials::{ward_of_renewal, ward_of_vengeance, ward_of_aegis, ward_of_sanctification};

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
        MonsterTrait::Stonekin(StonekinTrait::Slateblade) => cliffs_edge,
        MonsterTrait::Stonekin(StonekinTrait::Pebblebound) => quaking_dodge,
        MonsterTrait::Stonekin(StonekinTrait::Bolderfist) => shared_earth_armor,
        MonsterTrait::Stonekin(StonekinTrait::Mountainheart) => titanic_retaliation,

        MonsterTrait::Celestial(CelestialTrait::Radiantheart) => ward_of_renewal,
        MonsterTrait::Celestial(CelestialTrait::Aetherwing) => ward_of_vengeance,
        MonsterTrait::Celestial(CelestialTrait::Aurenguard) => ward_of_aegis,
        MonsterTrait::Celestial(CelestialTrait::Divinarch) => ward_of_sanctification,
    }
}
