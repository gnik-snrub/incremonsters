use rand::{Rng, thread_rng};

use super::{MonsterTrait, Trait};
use crate::{math::battle::damage_calculation, models::{Monster, Trigger}};

#[derive(Clone, Copy)]
pub enum StonekinTrait {
    Slateblade,
    Pebblebound,
    Bolderfist,
    Mountainheart,
}

impl MonsterTrait for StonekinTrait {
    fn get(&self) -> Trait {
        match self {
            StonekinTrait::Slateblade => {
                Trait {
                    name: "Cliff's Edge".to_string(),
                    description: "Defense stat (50%) provides extra damage on hit by reinforcing its powerful blows".to_string(),
                    trigger: Trigger::OnAttack,
                    callback: cliffs_edge
                }
            }
            StonekinTrait::Pebblebound => {
                Trait {
                    name: "Quaking Dodge".to_string(),
                    description: "Quick footwork and powerful stomping allows it a chance to dodge attacks at higher defense, while weakening the defense of enemies".to_string(),
                    trigger: Trigger::OnDefend,
                    callback: quaking_dodge
                }
            }
            StonekinTrait::Bolderfist => {
                Trait {
                    name: "Shared Earth Armor".to_string(),
                    description: "On hit, offers shards of carapace to allies, adding its own defense (20%) to that of allies.".to_string(),
                    trigger: Trigger::OnDamage,
                    callback: shared_earth_armor
                }
            }
            StonekinTrait::Mountainheart => {
                Trait {
                    name: "Titanic Retaliation".to_string(),
                    description: "When hit, lets out a seismic shock, dealing damage to all who oppose it based on its defense (50%)".to_string(),
                    trigger: Trigger::OnDamage,
                    callback: titanic_retaliation
                }
            }
        }
    }
}

fn cliffs_edge(
    self_value: Option<Monster>,
    _opponent: Option<Monster>,
    _allies: Option<Vec<Monster>>,
    _enemies: Option<Vec<Monster>>,
    damage: Option<i32>
) -> (Option<Monster>, Option<Monster>, Option<Vec<Monster>>, Option<Vec<Monster>>, Option<i32>) {
    let mut unwrapped_damage = damage.unwrap();
    let unwrapped_self = self_value.unwrap();
    unwrapped_damage += unwrapped_self.def / 2;

    (None, None, None, None, Some(unwrapped_damage))
}

fn quaking_dodge(
    self_value: Option<Monster>,
    opponent: Option<Monster>,
    _allies: Option<Vec<Monster>>,
    enemies: Option<Vec<Monster>>,
    damage: Option<i32>
) -> (Option<Monster>, Option<Monster>, Option<Vec<Monster>>, Option<Vec<Monster>>, Option<i32>) {
    let unwrapped_self = self_value.unwrap();
    let unwrapped_opponent = opponent.unwrap();
    let mut unwrapped_enemies = enemies.unwrap();
    let mut unwrapped_damage = damage.unwrap();

    let def_cubed = unwrapped_self.atk.pow(3);
    let atk_cubed = unwrapped_opponent.atk.pow(3);

    let probability: f64 = ((def_cubed / (def_cubed + atk_cubed)) as f64).clamp(0.1, 0.9);

    if thread_rng().gen_bool(probability.into()) {
        unwrapped_damage = 0;
        for enemy in &mut unwrapped_enemies {
            enemy.def -= enemy.def / 20;
        }
    }

    (None, None, None, Some(unwrapped_enemies), Some(unwrapped_damage))
}

fn shared_earth_armor(
    self_value: Option<Monster>,
    _opponent: Option<Monster>,
    allies: Option<Vec<Monster>>,
    _enemies: Option<Vec<Monster>>,
    _damage: Option<i32>
) -> (Option<Monster>, Option<Monster>, Option<Vec<Monster>>, Option<Vec<Monster>>, Option<i32>) {
    let unwrapped_self = self_value.unwrap();
    let mut unwrapped_allies = allies.unwrap();

    let def_bonus = unwrapped_self.def / 5;
    for ally in &mut unwrapped_allies {
        ally.def += def_bonus;
    }

    (None, None, Some(unwrapped_allies), None, None)
}

fn titanic_retaliation(
    self_value: Option<Monster>,
    _opponent: Option<Monster>,
    _allies: Option<Vec<Monster>>,
    enemies: Option<Vec<Monster>>,
    _damage: Option<i32>
) -> (Option<Monster>, Option<Monster>, Option<Vec<Monster>>, Option<Vec<Monster>>, Option<i32>) {
    let unwrapped_self = self_value.unwrap();
    let mut unwrapped_enemies = enemies.unwrap();

    let retaliation_damage = unwrapped_self.def / 2;

    for enemy in &mut unwrapped_enemies {
        enemy.damage += damage_calculation(retaliation_damage, enemy.def);
    }

    (None, None, None, Some(unwrapped_enemies), None)
}
