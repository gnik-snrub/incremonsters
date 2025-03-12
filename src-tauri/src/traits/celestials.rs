use rand::{Rng, thread_rng};
use serde::{Deserialize, Serialize};

use super::{MonsterTrait, Trait, TraitTrait};
use crate::{math::battle::damage_calculation, models::{CallbackFn, ModMode, ModType, Monster, TemporaryModifier, Trigger}};

use super::MonsterTrait::Celestial;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Deserialize, Serialize)]
pub enum CelestialTrait {
    Radiantheart,
    Aetherwing,
    HaloSentinel,
    Divinarch,
}

impl TraitTrait for CelestialTrait {
    fn create(self) -> Trait {
        match self {
            CelestialTrait::Radiantheart => {
                Trait {
                    name: "Ward of Renewal".to_string(),
                    description: "Radiantheart bestows a ward on its allies, healing them based on its damage taken (25%)".to_string(),
                    trigger: Trigger::OnAttack,
                    monster: Celestial(CelestialTrait::Radiantheart),
                }
            }
            CelestialTrait::Aetherwing => {
                Trait {
                    name: "Ward of Vengeance".to_string(),
                    description: "Aetherwing bestows a ward on its allies, dealing damage to their attackers based on its attack (30%)".to_string(),
                    trigger: Trigger::OnAttack,
                    monster: Celestial(CelestialTrait::Aetherwing),
                }
            }
            CelestialTrait::HaloSentinel => {
                Trait {
                    name: "Ward of Aegis".to_string(),
                    description: "Halo Sentinel bestows a ward on its allies, granting them passive damage reductions (-15%)".to_string(),
                    trigger: Trigger::OnAttack,
                    monster: Celestial(CelestialTrait::HaloSentinel),
                }
            }
            CelestialTrait::Divinarch => {
                Trait {
                    name: "Ward of Sanctification".to_string(),
                    description: "Divinarch bestows a ward on its allies, healing them each turn based on their highest stat (10% - ATK/DEF/SPD)".to_string(),
                    trigger: Trigger::OnAttack,
                    monster: Celestial(CelestialTrait::Divinarch),
                }
            }
        }
    }

    fn get(&self) -> CallbackFn {
        match self {
            CelestialTrait::Radiantheart => {
                ward_of_renewal
            }
            CelestialTrait::Aetherwing => {
                ward_of_vengeance
            }
            CelestialTrait::HaloSentinel => {
                ward_of_aegis
            }
            CelestialTrait::Divinarch => {
                ward_of_sanctification
            }
        }
    }
}


