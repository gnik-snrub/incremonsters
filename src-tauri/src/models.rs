// src/models.rs
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct Monster {
    pub id: String,
    pub name: String,
    pub hp: i32,
    pub current_hp: i32,
    pub atk: i32,
    pub def: i32,
    pub spd: i32,
    pub exp: i32,
    pub lvl: i32,
    pub traits: Vec<Trait>,
    pub temporary_modifiers: Vec<TemporaryModifier>,
}

enum ModMode {
  Additive,
  Multiplicative,
}

enum ModType {
  HP,
  ATK,
  DEF,
  SPD,
}

pub struct TemporaryModifier {
    source: String,
    mod_type: ModType,
    mod_mode: ModMode,
    mod_value: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum Trigger {
    OnHit,
    OnDamage,
    StartOfTurn,
    EndOfTurn,
}

type CallbackFn = fn(
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
    name: String,
    description: String,
    trigger: Trigger,
    #[serde(skip)]
    #[serde(default = "default_callback")]
    callback: CallbackFn,
}

impl Monster {
    pub fn new(name: &str, hp: i32, atk: i32, def: i32, spd: i32) -> Self {
        Monster {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            hp,
            current_hp: hp,
            atk,
            def,
            spd,
            exp: 0,
            lvl: 1,
            traits: Vec::new()
        }
    }

    pub fn add_trait(&mut self, trait_: Trait) {
        self.traits.push(trait_);
    }

    pub fn trigger_traits(
        &self,
        trigger: Trigger,
        allies: Option<Vec<Monster>>,
        enemies: Option<Vec<Monster>>,
    ) -> (Option<Vec<Monster>>, Option<Vec<Monster>>) {
        let mut current_allies = allies;
        let mut current_enemies = enemies;

        for trait_ in &self.traits {
            if trait_.trigger == trigger {
                let (new_allies, new_enemies) = (trait_.callback)(current_allies, current_enemies);
                current_allies = new_allies;
                current_enemies = new_enemies;
            }
        }

        (current_allies, current_enemies)
    }
}
