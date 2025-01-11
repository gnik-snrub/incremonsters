mod models;

use std::fs;

use models::Monster;
mod monsters;
use monsters::level_up;
use monsters::*;
mod math;
use math::battle::battle;
use math::rewards::win_battle_rewards;

use rand::Rng;

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
            save,
            load
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
