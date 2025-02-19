use crate::models::Monster;
use serde::{Deserialize, Serialize};
use rand::seq::SliceRandom;
use rand::{random};

fn get_speed_order(player: &Vec<Monster>, enemy: &Vec<Monster>, player_speed_mod: f32) -> Vec<(i32, String, usize)> {
    let mut combined: Vec<(i32, String, usize)> = Vec::new();
    for (i, monster) in player.iter().enumerate() {
        combined.push(((monster.spd as f32 * player_speed_mod) as i32, "player".to_string(), i));
    }
    for (i, monster) in enemy.iter().enumerate() {
        combined.push((monster.spd, "enemy".to_string(), i));
    }
    combined.sort_by(|a, b| {
        let cmp = a.0.cmp(&b.0);
        if cmp == std::cmp::Ordering::Equal {
            if random::<bool>() {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        } else {
            cmp
        }
    });
    combined
}

fn get_target(monsters: &Vec<Monster>, hp_mod: f32) -> Option<usize> {
    let mut rng = rand::thread_rng();
    let mut alive: Vec<usize> = monsters
        .iter()
        .enumerate()
        .filter_map(|(idx, m)| {
            if m.damage < m.hp {
                Some(idx)
            } else {
                None
            }
        }).collect::<Vec<_>>();

    alive.shuffle(&mut rng);
    if alive.is_empty() {
        return None;
    }
    return Some(alive[0]);
}

pub fn damage_calculation(atk: i32, def: i32) -> i32 {
    if atk <= 0 {
        return 0;
    }

    let f32_atk: f32 = atk as f32;
    let f32_def: f32 = def as f32;

    let divisor: f32 = if def <= 0 {
        f32_atk
    } else {
        f32_atk + f32_def
    };
    let calculated: f32 = f32_atk * (f32_atk / divisor);
    calculated.ceil() as i32
}

#[derive(Serialize, Deserialize)]
pub struct GlobalModifier {
    sourceId: String,
    name: String,
    description: String,
    modType: ModType,
    modValue: f32,
}

#[derive(Serialize, Deserialize)]
pub enum ModType {
  MULT,
  ADD,
  SUB,
  DIV,
}

#[derive(Serialize, Deserialize)]
pub struct GlobalModifiers {
    atk: Vec<GlobalModifier>,
    def: Vec<GlobalModifier>,
    spd: Vec<GlobalModifier>,
    hp: Vec<GlobalModifier>,
}

fn calculate_modifier(mods: Vec<GlobalModifier>) -> f32 {
    let mut mod_total: f32 = 1.0;
    for m in mods {
        match m.modType {
            ModType::ADD => {
                mod_total += m.modValue;
            },
            ModType::SUB => {
                mod_total -= m.modValue;
            },
            ModType::DIV => {
                mod_total /= m.modValue;
            },
            ModType::MULT => {
                mod_total *= m.modValue;
            },
        }
    }
    mod_total
}

#[tauri::command]
pub fn battle(mut player: Vec<Monster>, mut enemy: Vec<Monster>, global_modifiers: GlobalModifiers) -> [Vec<Monster>; 2] {
    let attack_mod = calculate_modifier(global_modifiers.atk);
    let defense_mod = calculate_modifier(global_modifiers.def);
    let speed_mod = calculate_modifier(global_modifiers.spd);
    let hp_mod = calculate_modifier(global_modifiers.hp);
    let ordered: Vec<(i32, String, usize)> = get_speed_order(&player, &enemy, speed_mod);
    for (_, side, index) in ordered {
        if side == "player" {
            match get_target(&enemy, 1.0) {
                Some(target_idx) => {
                    let damage: i32 = damage_calculation((player[index].atk as f32 * attack_mod) as i32, enemy[target_idx].def);
                    enemy[target_idx].damage += damage;
                    if enemy[target_idx].damage > enemy[target_idx].hp {
                        enemy[target_idx].damage = enemy[target_idx].hp;
                    }
                }
                None => continue,
            }
        } else {
            match get_target(&player, hp_mod) {
                Some(target_idx) => {
                    let damage: i32 = damage_calculation(enemy[index].atk, (player[target_idx].def as f32 * defense_mod) as i32);
                    player[target_idx].damage += damage;
                    if player[target_idx].damage as f32 > player[target_idx].hp as f32 * hp_mod {
                        player[target_idx].damage = player[target_idx].hp;
                    }
                }
                None => continue,
            }
        }
    }
    [player, enemy]
}
