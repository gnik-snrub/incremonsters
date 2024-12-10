mod models;

use models::Monster;

use rand::Rng;

#[tauri::command]
fn process_battle() {
    // TODO
}

#[tauri::command]
fn create_monster() -> Monster {
    let mut rng = rand::thread_rng();
    let hp: i32 = rng.gen_range(0..100);
    let atk: i32 = rng.gen_range(0..100);
    let def: i32 = rng.gen_range(0..100);
    let spd: i32 = rng.gen_range(0..100);

    println!("HP: {}", hp);
    println!("ATK: {}", atk);
    println!("DEF: {}", def);
    println!("SPD: {}", spd);

    Monster {
        name: "Monster".to_string(),
        hp,
        current_hp: hp,
        atk,
        def,
        spd,
        exp: 0,
        lvl: 1,
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![process_battle, create_monster])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
