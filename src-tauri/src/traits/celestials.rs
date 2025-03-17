use rand::{Rng, thread_rng};
use serde::{Deserialize, Serialize};

use super::{MonsterTrait, Trait, TraitTrait};
use crate::{math::battle::damage_calculation, models::{CallbackFn, ModMode, ModType, Monster, TemporaryModifier, Trigger}};

use super::MonsterTrait::Celestial;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Deserialize, Serialize)]
pub enum CelestialTrait {
    Radiantheart,
    Aetherwing,
    Aurenguard,
    Divinarch,
}

impl TraitTrait for CelestialTrait {
    fn create(self) -> Trait {
        match self {
            CelestialTrait::Radiantheart => {
                Trait {
                    name: "Ward of Renewal".to_string(),
                    description: "Radiantheart bestows a ward on its allies, healing them based on its damage taken (10%)".to_string(),
                    trigger: Trigger::OnAttack,
                    monster: Celestial(CelestialTrait::Radiantheart),
                }
            }
            CelestialTrait::Aetherwing => {
                Trait {
                    name: "Ward of Vengeance".to_string(),
                    description: "Aetherwings bestows a ward on its allies, increasing their attack stat based on their damage taken (20%)".to_string(),
                    trigger: Trigger::OnAttack,
                    monster: Celestial(CelestialTrait::Aetherwing),
                }
            }
            CelestialTrait::Aurenguard => {
                Trait {
                    name: "Ward of Aegis".to_string(),
                    description: "Aurenguard bestows a ward on its allies, increasing their defense stat based on their damage taken (20%)".to_string(),
                    trigger: Trigger::OnAttack,
                    monster: Celestial(CelestialTrait::Aurenguard),
                }
            }
            CelestialTrait::Divinarch => {
                Trait {
                    name: "Ward of Sacrifice".to_string(),
                    description: "If Divinarch has 90% or more health, it sends out a ward to a dead ally, healing them to 10% of their health, while lowering Divinarchs health the same amount".to_string(),
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
            CelestialTrait::Aurenguard => {
                ward_of_aegis
            }
            CelestialTrait::Divinarch => {
                ward_of_sanctification
            }
        }
    }
}


pub fn ward_of_renewal(
    self_value: Option<Monster>,
    _opponent: Option<Monster>,
    allies: Option<Vec<Monster>>,
    _enemies: Option<Vec<Monster>>,
    _damage: Option<i32>
) -> (Option<Monster>, Option<Monster>, Option<Vec<Monster>>, Option<Vec<Monster>>, Option<i32>) {
    let unwrapped_self = self_value.unwrap();
    let mut unwrapped_allies = allies.unwrap();

    let heal_value = unwrapped_self.damage / 10;
    for ally in &mut unwrapped_allies {
        if ally.damage >= ally.hp + ally.stat_adjustments.hp {
            continue;
        }
        ally.damage = std::cmp::min( ally.damage + heal_value, ally.hp + ally.stat_adjustments.hp);
    }

    (None, None, Some(unwrapped_allies), None, None)
}

pub fn ward_of_vengeance(
    _self_value: Option<Monster>,
    _opponent: Option<Monster>,
    allies: Option<Vec<Monster>>,
    _enemies: Option<Vec<Monster>>,
    damage: Option<i32>
) -> (Option<Monster>, Option<Monster>, Option<Vec<Monster>>, Option<Vec<Monster>>, Option<i32>) {
    let mut unwrapped_allies = allies.unwrap();

    for ally in &mut unwrapped_allies {
        if ally.damage >= ally.hp + ally.stat_adjustments.hp {
            continue;
        }
        let atk_boost = ally.damage / 5;
        if let Some(idx) = ally.temporary_modifiers.iter().position(|modifier| modifier.source == "ward_of_vengeance".to_string()) {
            let mut modifier = ally.temporary_modifiers[idx].clone();
            modifier.mod_value += atk_boost;
            ally.temporary_modifiers[idx] = modifier;
        } else {
            let modifier = TemporaryModifier {
                source: "ward_of_vengeance".to_string(),
                mod_type: ModType::ATK,
                mod_mode: ModMode::Add,
                mod_value: atk_boost,
                quantity: 1,
            };
            ally.temporary_modifiers.push(modifier);
        }
    }

    (None, None, Some(unwrapped_allies), None, None)
}

pub fn ward_of_aegis(
    _self_value: Option<Monster>,
    _opponent: Option<Monster>,
    allies: Option<Vec<Monster>>,
    _enemies: Option<Vec<Monster>>,
    _damage: Option<i32>
) -> (Option<Monster>, Option<Monster>, Option<Vec<Monster>>, Option<Vec<Monster>>, Option<i32>) {
    let mut unwrapped_allies = allies.unwrap();

    for ally in &mut unwrapped_allies {
        if ally.damage >= ally.hp + ally.stat_adjustments.hp {
            continue;
        }
        let def_boost = ally.damage / 5;
        if let Some(idx) = ally.temporary_modifiers.iter().position(|modifier| modifier.source == "ward_of_aegis".to_string()) {
            let mut modifier = ally.temporary_modifiers[idx].clone();
            modifier.mod_value += def_boost;
            ally.temporary_modifiers[idx] = modifier;
        } else {
            let modifier = TemporaryModifier {
                source: "ward_of_aegis".to_string(),
                mod_type: ModType::DEF,
                mod_mode: ModMode::Add,
                mod_value: def_boost,
                quantity: 1,
            };
            ally.temporary_modifiers.push(modifier);
        }
    }

    (None, None, Some(unwrapped_allies), None, None)
}

pub fn ward_of_sanctification(
    self_value: Option<Monster>,
    _opponent: Option<Monster>,
    allies: Option<Vec<Monster>>,
    _enemies: Option<Vec<Monster>>,
    damage: Option<i32>
) -> (Option<Monster>, Option<Monster>, Option<Vec<Monster>>, Option<Vec<Monster>>, Option<i32>) {

    (None, None, None, None, None)
}
