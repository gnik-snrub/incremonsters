use crate::models::Monster;
use crate::monsters::GrowthRates;

pub fn new() -> Monster {
    Monster::new("zombie", 150, 10, 15, 5)
}

pub const ZOMBIE_GROWTH_RATE: GrowthRates = GrowthRates {
    hp: 1.05,
    atk: 1.01,
    def: 1.04,
    spd: 1.01
};
