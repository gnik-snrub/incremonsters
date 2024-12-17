use crate::models::Monster;
use crate::monsters::GrowthRates;

pub fn new() -> Monster {
    Monster::new("zombie", 150, 10, 15, 5)
}

pub const ZOMBIE_GROWTH_RATE: GrowthRates = GrowthRates {
    hp: 1.5,
    atk: 1.1,
    def: 1.4,
    spd: 1.1
};
