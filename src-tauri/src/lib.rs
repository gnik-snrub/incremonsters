mod models;
use models::Monster;
mod monsters;
use monsters::*;
mod math;
use math::damage::damage_calculation;

use rand::Rng;

#[tauri::command]
fn process_battle() {
    // TODO
}

#[tauri::command]
fn create_monster() -> Monster {
    let monster_options: [fn() -> Monster; 3] = [skeleton::new, ogre::new, zombie::new];
    let mut rng = rand::thread_rng();
    monster_options[rng.gen_range(0..monster_options.len())]()
}

#[tauri::command]
fn calculate_damage(atk: i32, mut def: i32) -> i32 {
    let output: i32 = damage_calculation(atk, def);
    output
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![process_battle, create_monster, calculate_damage])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
