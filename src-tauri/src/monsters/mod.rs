use ogre::OGRE_GROWTH_RATE;
use skeleton::SKELETON_GROWTH_RATE;
use zombie::ZOMBIE_GROWTH_RATE;

use crate::models::Monster;

pub mod skeleton;
pub mod ogre;
pub mod zombie;

pub struct GrowthRates {
    pub hp: f32,
    pub atk: f32,
    pub def: f32,
    pub spd: f32
}

const MISSING_GROWTH_RATE: GrowthRates = GrowthRates {
    hp: 0.0,
    atk: 0.0,
    def: 0.0,
    spd: 0.0
};

pub fn level_up(monster: &mut Monster) {
    let growth_rates: GrowthRates = find_growth_rate(monster.clone());
    monster.lvl += 1;
    monster.hp = (monster.hp as f32 * growth_rates.hp).ceil() as i32;
    monster.atk = (monster.atk as f32 * growth_rates.atk).ceil() as i32;
    monster.def = (monster.def as f32 * growth_rates.def).ceil() as i32;
    monster.spd = (monster.spd as f32 * growth_rates.spd).ceil() as i32;
}

fn find_growth_rate(monster: Monster) -> GrowthRates {
    match monster.name.as_str() {
        "skeleton" => SKELETON_GROWTH_RATE,
        "ogre" => OGRE_GROWTH_RATE,
        "zombie" => ZOMBIE_GROWTH_RATE,
        _ => MISSING_GROWTH_RATE
    }
}
