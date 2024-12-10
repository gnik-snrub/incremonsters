// src/models.rs
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Monster {
    pub name: String,
    pub hp: i32,
    pub current_hp: i32,
    pub atk: i32,
    pub def: i32,
    pub spd: i32,
    pub exp: i32,
    pub lvl: i32,
}

