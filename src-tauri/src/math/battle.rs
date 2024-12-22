use crate::models::Monster;
use rand::{Rng, random};

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

const NO_TARGET: usize = 5;

fn get_target(monsters: &Vec<Monster>) -> usize {
    let mut rng = rand::thread_rng();
    let mut target: usize;
    let mut target_list: Vec<usize> = vec![];
    loop {
        if target_list.len() == 4 {
            target = NO_TARGET;
            break;
        }
        target = rng.gen_range(0..4);
        if target_list.contains(&target) {
            continue;
        }
        if monsters[target].hp > 0 {
            break;
        }
        target_list.push(target);
    }
    target
}

fn attack(attacker: &Monster, target: &mut Monster) {
    if attacker.hp <= 0 {
        return;
    }
    let damage: i32 = damage_calculation(attacker.atk, target.def);
    target.hp = std::cmp::max(target.hp - damage, 0) as i32;
}

pub fn damage_calculation(atk: i32, def: i32) -> i32 {
    let calculated: f32 = atk as f32 * ((atk as f32) / if (def) <= 0 { atk } else { atk + def } as f32);
    calculated.round() as i32
}

#[tauri::command]
pub fn battle(mut player: Vec<Monster>, mut enemy: Vec<Monster>) -> [Vec<Monster>; 2] {
    let ordered: Vec<(i32, String, usize)> = get_speed_order(&player, &enemy);
    for (_, side, index) in ordered {
        if side == "player" {
            let target_idx = get_target(&enemy);
            if target_idx == NO_TARGET {
                continue;
            }
            let target = &mut enemy[target_idx];
            attack(&player[index], target);
        } else {
            let target_idx = get_target(&player);
            if target_idx == NO_TARGET {
                continue;
            }
            let target = &mut player[target_idx];
            attack(&enemy[index], target);
        }
    }
    [player, enemy]
}
