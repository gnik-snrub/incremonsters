// src/models.rs
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::traits::{get_callback, MonsterTrait, TraitTrait};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct Monster {
    pub id: String,
    pub creature_family: String,
    pub creature_type: String,
    pub name: String,
    pub hp: i32,
    pub damage: i32,
    pub atk: i32,
    pub def: i32,
    pub spd: i32,
    pub exp: i32,
    pub lvl: i32,
    pub traits: Vec<Trait>,
    pub temporary_modifiers: Vec<TemporaryModifier>,
    pub stat_adjustments: StatAdjustments,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct StatAdjustments {
    pub hp: i32,
    pub atk: i32,
    pub def: i32,
    pub spd: i32,
    pub dmg: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum ModMode {
  Add,
  Mult,
  Sub,
  Div,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum ModType {
  HP,
  ATK,
  DEF,
  SPD,
  DMG,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct TemporaryModifier {
    pub source: String,
    pub mod_type: ModType,
    pub mod_mode: ModMode,
    pub mod_value: i32,
    pub quantity: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum Trigger {
    OnAttack, //Attack is started, before damage calc
    OnHit, //Attack has landed, after damage calc
    OnDefend, //Has been attacked, after damage calc, before damage applied
    OnDamage, //Has been damaged, after damage applied
    StartOfTurn,
    EndOfTurn,
}

pub type CallbackFn = fn(
    self_value: Option<Monster>,
    opponent: Option<Monster>,
    allies: Option<Vec<Monster>>,
    enemies: Option<Vec<Monster>>,
    damage: Option<i32>,
) -> (Option<Monster>, Option<Monster>, Option<Vec<Monster>>, Option<Vec<Monster>>, Option<i32>);

fn default_callback() -> CallbackFn {
    |_: Option<Monster>, _: Option<Monster>, _: Option<Vec<Monster>>, _: Option<Vec<Monster>>, _: Option<i32>| (None, None, None, None, None)
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct Trait {
    pub name: String,
    pub description: String,
    pub trigger: Trigger,
    pub monster: MonsterTrait,
}

impl Monster {
    pub fn new(creature_family: &str, creature_type: &str, hp: i32, atk: i32, def: i32, spd: i32) -> Self {
        Monster {
            id: Uuid::new_v4().to_string(),
            creature_family: creature_family.to_string(),
            creature_type: creature_type.to_string(),
            name: format!("{} {}", creature_family, creature_type),
            hp,
            damage: 0,
            atk,
            def,
            spd,
            exp: 0,
            lvl: 1,
            traits: Vec::new(),
            temporary_modifiers: Vec::new(),
            stat_adjustments: StatAdjustments {
                hp: 0,
                atk: 0,
                def: 0,
                spd: 0,
                dmg: 0,
            }
        }
    }

    pub fn add_trait(&mut self, trait_: Trait) {
        self.traits.push(trait_);
    }

    pub fn trigger_traits(
        &self,
        trigger: Trigger,
        opponent: &Option<Monster>,
        allies: Option<Vec<Monster>>,
        enemies: Option<Vec<Monster>>,
        damage: Option<i32>,
    ) -> (Option<Monster>, Option<Monster>, Option<Vec<Monster>>, Option<Vec<Monster>>, Option<i32>) {
        let mut current_self = Some(self.clone());
        let mut current_opponent = opponent.clone();
        let mut current_allies = allies;
        let mut current_enemies = enemies;
        let mut current_damage = damage;

        for trait_ in self.clone().traits {
            if trait_.trigger == trigger {
                let callback = get_callback(trait_.monster);
                let (new_self,
                    new_opponent,
                    new_allies,
                    new_enemies,
                    new_damage
                    ) = (callback)(Some(self.clone()), opponent.clone(), current_allies.clone(), current_enemies.clone(), current_damage);

                current_self =  match new_self {
                    Some(new_self) => Some(new_self),
                    None => current_self
                };
                current_opponent = match new_opponent {
                    Some(new_opponent) => Some(new_opponent),
                    None => current_opponent
                };
                current_allies = match new_allies {
                    Some(new_allies) => Some(new_allies),
                    None => current_allies
                };
                current_enemies = match new_enemies {
                    Some(new_enemies) => Some(new_enemies),
                    None => current_enemies
                };
                current_damage = match new_damage {
                    Some(new_damage) => Some(new_damage),
                    None => current_damage
                };

            }
        }

        (current_self, current_opponent, current_allies, current_enemies, current_damage)
    }
}
