use ogre::OGRE_GROWTH_RATE;
use skeleton::SKELETON_GROWTH_RATE;
use stonekin::StonekinType;
use zombie::ZOMBIE_GROWTH_RATE;

use crate::{math::rewards::{GrowthBoosts, GrowthModifier}, models::Monster};

pub mod ogre;
pub mod skeleton;
pub mod zombie;
pub mod stonekin;

pub enum MonsterFamily {
    Stonekin(Option<StonekinType>),
}

pub trait MonsterType {
    fn generate(&self) -> Monster;
    fn random() -> Self;
}

pub struct GrowthRates {
    pub hp: f32,
    pub atk: f32,
    pub def: f32,
    pub spd: f32,
}

const MISSING_GROWTH_RATE: GrowthRates = GrowthRates {
    hp: 0.0,
    atk: 0.0,
    def: 0.0,
    spd: 0.0,
};

pub fn level_up(monster: &mut Monster, modifiers: &GrowthBoosts) {
    let growth_rates: GrowthRates = find_growth_rate(monster.clone());
    monster.lvl += 1;
    monster.hp = get_new_stat(monster.hp, growth_rates.hp, &modifiers.0);
    monster.atk = get_new_stat(monster.atk, growth_rates.atk, &modifiers.1);
    monster.def = get_new_stat(monster.def, growth_rates.def, &modifiers.2);
    monster.spd = get_new_stat(monster.spd, growth_rates.spd, &modifiers.3);
    monster.current_hp = monster.hp;
}

fn get_new_stat(original_stat: i32, growth_rate: f32, modifiers: &Vec<GrowthModifier>) -> i32 {
    let mut new_stat = original_stat as f32;
    let mut final_growth_rate = growth_rate;

    for modifier in modifiers {
        if modifier.operation == "add" {
            final_growth_rate += modifier.magnitude;
        } else if modifier.operation == "mult" {
            final_growth_rate *= modifier.magnitude;
        }
    }
    new_stat = (new_stat * final_growth_rate).ceil();
    new_stat as i32
}

fn find_growth_rate(monster: Monster) -> GrowthRates {
    match monster.name.as_str() {
        "skeleton" => SKELETON_GROWTH_RATE,
        "ogre" => OGRE_GROWTH_RATE,
        "zombie" => ZOMBIE_GROWTH_RATE,
        _ => MISSING_GROWTH_RATE,
    }
}

fn generate_monster(family: MonsterFamily) -> Monster {
    match family {
        MonsterFamily::Stonekin(None) => StonekinType::random().generate(),
        MonsterFamily::Stonekin(Some(monster)) => monster.generate(),
    }
}
