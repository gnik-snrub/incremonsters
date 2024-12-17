use crate::models::Monster;
use crate::monsters::GrowthRates;

pub fn new() -> Monster {
    Monster::new("skeleton", 80, 11, 9, 15)
}

pub const SKELETON_GROWTH_RATE: GrowthRates = GrowthRates {
    hp: 1.4,
    atk: 1.2,
    def: 1.1,
    spd: 1.4
};
