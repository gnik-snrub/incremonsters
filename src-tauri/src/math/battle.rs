use crate::models::Monster;
use rand::seq::SliceRandom;
use rand::{random};

fn get_speed_order(player: &Vec<Monster>, enemy: &Vec<Monster>) -> Vec<(i32, String, usize)> {
    let mut combined: Vec<(i32, String, usize)> = Vec::new();
    for (i, monster) in player.iter().enumerate() {
        combined.push((monster.spd, "player".to_string(), i));
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

fn get_target(monsters: &Vec<Monster>) -> Option<usize> {
    let mut rng = rand::thread_rng();
    let alive: Vec<&Monster> = monsters.iter().filter(|m| m.hp > 0).collect();
    let mut target_list: Vec<usize> = (0..alive.len()).collect();
    target_list.shuffle(&mut rng);

    for &i in target_list.iter() {
        return Some(i);
    }
    None
}

pub fn damage_calculation(atk: i32, def: i32) -> i32 {
    if atk <= 0 {
        return 0;
    }
    let divisor: f32 = if def <= 0 {
        atk as f32
    } else {
        (atk + def) as f32
    };
    let calculated: f32 = (atk as f32) * (atk as f32 / divisor);
    calculated.round() as i32
}

#[tauri::command]
pub fn battle(mut player: Vec<Monster>, mut enemy: Vec<Monster>) -> [Vec<Monster>; 2] {
    let ordered: Vec<(i32, String, usize)> = get_speed_order(&player, &enemy);
    for (_, side, index) in ordered {
        if side == "player" {
            match get_target(&enemy) {
                Some(target_idx) => {
                    let damage: i32 = damage_calculation(player[index].atk, enemy[target_idx].def);
                    enemy[target_idx].current_hp =
                        std::cmp::max(enemy[target_idx].current_hp - damage, 0) as i32;
                }
                None => continue,
            }
        } else {
            match get_target(&player) {
                Some(target_idx) => {
                    let damage: i32 = damage_calculation(enemy[index].atk, player[target_idx].def);
                    player[target_idx].current_hp =
                        std::cmp::max(player[target_idx].current_hp - damage, 0) as i32;
                }
                None => continue,
            }
        }
    }
    [player, enemy]
}
