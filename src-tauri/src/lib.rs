mod models;

use models::Monster;
mod monsters;
use monsters::level_up;
use monsters::*;
mod math;
use math::battle::battle;
use math::rewards::win_battle_rewards;

use rand::Rng;
use serde::{Deserialize, Serialize};

#[tauri::command]
fn process_battle() {
    // TODO
}

#[tauri::command]
fn create_monster(lvl: i32) -> Monster {
    let monster_options: [fn() -> Monster; 3] = [skeleton::new, ogre::new, zombie::new];
    let mut rng = rand::thread_rng();
    let mut monster = monster_options[rng.gen_range(0..monster_options.len())]();
    for _ in 1..lvl {
        level_up(&mut monster);
    }
    monster
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            process_battle,
            create_monster,
            battle,
            win_battle_rewards,
            save
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
