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

#[tauri::command]
pub fn battle(mut player: Vec<Monster>, mut enemy: Vec<Monster>) -> [Vec<Monster>; 2] {
    let mut player_targets: Vec<usize> = get_target();
    let mut enemy_targets: Vec<usize> = get_target();
    [player, enemy]
}
