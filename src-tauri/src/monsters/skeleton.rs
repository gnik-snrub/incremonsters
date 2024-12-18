use crate::models::Monster;
use crate::monsters::GrowthRates;

pub fn new() -> Monster {
    Monster::new("skeleton", 80, 11, 9, 15)
}

pub const SKELETON_GROWTH_RATE: GrowthRates = GrowthRates {
    hp: 1.04,
    atk: 1.02,
    def: 1.01,
    spd: 1.04
};
