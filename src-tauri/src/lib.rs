mod models;

use std::fs;
use rand::seq::SliceRandom;

use models::Monster;
mod monsters;
use monsters::level_up;
use monsters::{MonsterFamily, generate_monster};
mod math;
use math::battle::battle;
use math::rewards::{win_battle_rewards, GrowthBoosts};
mod traits;
pub mod image_processing;
use image_processing::generate_fusion_sprite;

#[tauri::command]
fn create_monster(lvl: i32) -> Monster {
    let monster_options = vec![MonsterFamily::Celestial(None), MonsterFamily::Stonekin(None)];
    let choice = monster_options.choose(&mut rand::thread_rng()).unwrap();
    let mut monster = generate_monster(choice.clone());
    for _ in 1..lvl {
        level_up(&mut monster, &GrowthBoosts(vec![], vec![], vec![], vec![]));
    }
    monster
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            create_monster,
            battle,
            win_battle_rewards,
            save,
            load,
            generate_fusion_sprite,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

extern crate directories;
use directories::UserDirs;

#[tauri::command]
fn save(data: String) {
    if let Some(dirs) = UserDirs::new() {
        if let Some(path) = dirs.document_dir() {
            println!("Document path: {:?}", path.join("/Saved Games/Incremonsters/savefile.txt"));
            println!("Data: {:?}", data);
            //fs::write(path.join("/Saved Games/Incremonsters/savefile.txt"), data);
        } else {
            let save_path = dirs.home_dir().join("Documents/Incremonsters/savefile.txt");

            if let Some(parent) = save_path.parent() {
                fs::create_dir_all(parent).expect("Couldn't create directory");
            }

            fs::write(&save_path, data).expect("Couldn't write to file");
        }
    } else {
        println!("No user dirs");
    }
}

#[tauri::command]
fn load() -> String {
    if let Some(dirs) = UserDirs::new() {
        if let Some(path) = dirs.document_dir() {
            return fs::read_to_string(path.join("/Saved Games/Incremonsters/savefile.txt")).expect("Couldn't read file");
        } else {
            let save_path = dirs.home_dir().join("Documents/Incremonsters/savefile.txt");
            return fs::read_to_string(&save_path).expect("Couldn't read file");
        }
    } else {
        println!("No user dirs");
        return "".to_string();
    }
}
