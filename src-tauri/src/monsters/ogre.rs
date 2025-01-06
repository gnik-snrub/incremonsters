use crate::models::Monster;
use crate::monsters::GrowthRates;

pub fn new() -> Monster {
    Monster::new("ogre", 100, 11, 15, 9)
}

pub const OGRE_GROWTH_RATE: GrowthRates = GrowthRates {
    hp: 1.05,
    atk: 1.02,
    def: 1.03,
    spd: 1.01,
};
