use crate::models::Monster;
use crate::monsters::GrowthRates;

pub enum StonekinType {
    Slateblade,
    Pebblebound,
    Bolderfist,
    Mountainheart,
}

pub fn new(type_: StonekinType) -> Monster {
    match type_ {
        StonekinType::Slateblade => new_slateblade(),
        StonekinType::Pebblebound => new_pebblebound(),
        StonekinType::Bolderfist => new_bolderfist(),
        StonekinType::Mountainheart => new_mountainheart(),
    }
}

pub fn get_growth_rate(type_: StonekinType) -> GrowthRates {
    match type_ {
        StonekinType::Slateblade => SLATEBLADE_GROWTH_RATE,
        StonekinType::Pebblebound => PEBBLEBOUND_GROWTH_RATE,
        StonekinType::Bolderfist => BOLDERFIST_GROWTH_RATE,
        StonekinType::Mountainheart => MOUNTAINHEART_GROWTH_RATE,
    }
}

pub fn new_slateblade() -> Monster {
    Monster::new("Slateblade", 100, 17, 15, 18)
}

pub fn new_pebblebound() -> Monster {
    Monster::new("Pebblebound", 100, 12, 18, 20)
}

pub fn new_bolderfist() -> Monster {
    Monster::new("Bolderfist", 100, 15, 25, 10)
}

pub fn new_mountainheart() -> Monster {
    Monster::new("Mountainheart", 110, 20, 20, 8)
}

pub const SLATEBLADE_GROWTH_RATE: GrowthRates = GrowthRates {
    hp: 1.05,
    atk: 1.045,
    def: 1.035,
    spd: 1.05,
};

pub const PEBBLEBOUND_GROWTH_RATE: GrowthRates = GrowthRates {
    hp: 1.05,
    atk: 1.02,
    def: 1.05,
    spd: 1.06,
};

pub const BOLDERFIST_GROWTH_RATE: GrowthRates = GrowthRates {
    hp: 1.05,
    atk: 1.035,
    def: 1.085,
    spd: 1.01,
};

pub const MOUNTAINHEART_GROWTH_RATE: GrowthRates = GrowthRates {
    hp: 1.05,
    atk: 1.06,
    def: 1.06,
    spd: 1.01,
};
