use crate::models::Monster;
use rand::{thread_rng, random};
use rand::seq::SliceRandom;

pub fn damage_calculation(atk: i32, def: i32) -> i32 {
    let calculated: f32 = atk as f32 * ((atk as f32) / if (def) <= 0 { atk } else { atk + def } as f32);
    calculated.round() as i32
}

fn get_target() -> Vec<usize> {
    let mut rng = thread_rng();
    let mut targets: Vec<usize> = vec![0, 1, 2, 3];
    targets.shuffle(&mut rng);
    targets
}

fn attack(attacker: &Monster, target: &mut Monster) {
    if attacker.hp <= 0 {
        return;
    }
    let damage: i32 = damage_calculation(attacker.atk, target.def);
    target.hp -= damage;
}

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

#[tauri::command]
pub fn battle(mut player: Vec<Monster>, mut enemy: Vec<Monster>) -> [Vec<Monster>; 2] {
    let mut player_targets: Vec<usize> = get_target();
    let mut enemy_targets: Vec<usize> = get_target();
    let ordered: Vec<(i32, String, usize)> = get_speed_order(&player, &enemy);
    [player, enemy]
}
