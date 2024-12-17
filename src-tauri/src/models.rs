// src/models.rs
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
pub struct Monster {
    pub id: String,
    pub name: String,
    pub hp: i32,
    pub current_hp: i32,
    pub atk: i32,
    pub def: i32,
    pub spd: i32,
    pub exp: i32,
    pub lvl: i32,
}

impl Monster {
    pub fn new(name: &str, hp: i32, atk: i32, def: i32, spd: i32) -> Self {
        Monster {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            hp,
            current_hp: hp,
            atk,
            def,
            spd,
            exp: 0,
            lvl: 1,
        }
    }
}
