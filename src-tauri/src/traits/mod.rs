use crate::models::Trait;

pub mod stonekin;

pub trait MonsterTrait {
    fn get(&self) -> Trait;
}
