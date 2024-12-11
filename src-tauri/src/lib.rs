mod models;
use models::Monster;
mod monsters;
use monsters::*;

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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![process_battle, create_monster])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
