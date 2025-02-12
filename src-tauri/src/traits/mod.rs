use crate::models::Trait;

pub mod stonekin;

pub struct Trait {
    name: String,
    description: String,
    trigger: Trigger,
    #[serde(skip)]
    #[serde(default = "default_callback")]
    callback: CallbackFn,
}

pub trait MonsterTrait {
    fn get(&self) -> Trait;
}
