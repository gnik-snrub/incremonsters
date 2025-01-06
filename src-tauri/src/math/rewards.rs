use crate::{models::Monster, monsters::level_up};
use rand::Rng;

const REWARD_BASE: f32 = 10.0;
const REWARD_GROWTH: f32 = 1.07;

fn get_rewards(dungeon_lvl: i32, slain: &Vec<Monster>) -> i32 {
    let mut rng = rand::thread_rng();
    let mut reward_total: f32 = 0.0;
    for monster in slain.iter() {
        let variance: f32 = rng.gen_range(0.8..1.2);
        reward_total +=
            (REWARD_BASE * REWARD_GROWTH.powf(monster.lvl as f32)) * variance * dungeon_lvl as f32;
    }
    ((reward_total * 100.0).round() / 100.0) as i32
}

fn get_exp(player_team_size: i32, average_team_level: i32, slain: Monster) -> i32 {
    let base_exp: f32 = (slain.hp + slain.atk + slain.spd + slain.def) as f32;
    let team_size_divisor: f32 = player_team_size as f32;
    let level_difference: f32 =
        (((2 * slain.lvl) + 10) as f32 / (slain.lvl + average_team_level + 10) as f32).powf(2.5);
    let exp: f32 = ((base_exp * slain.lvl as f32) / 5 as f32)
        * (1 as f32 / team_size_divisor)
        * level_difference
        + 1.0;
    exp as i32
}

fn calc_level_up(mut player: Vec<Monster>, exp: i32) -> Vec<Monster> {
    for mut monster in player.iter_mut() {
        let mut monsters_exp: i32 = exp + monster.exp;
        monster.exp = 0;
        while monsters_exp >= monster.lvl.pow(3) {
            monsters_exp -= monster.lvl.pow(3);
            level_up(&mut monster);
        }
        monster.exp = std::cmp::max(monsters_exp, 0);
    }
    player
}

#[tauri::command]
pub fn win_battle_rewards(
    dungeon_lvl: i32,
    mut player: Vec<Monster>,
    enemy: Vec<Monster>,
) -> (Vec<Monster>, i32) {
    let rewards: i32 = get_rewards(dungeon_lvl, &enemy);
    let mut exp_total: i32 = 0;
    let average_team_level: i32 = if player.is_empty() {
        1
    } else {
        player.iter().map(|m| m.lvl).sum::<i32>() / player.len() as i32
    };
    for monster in enemy.iter() {
        exp_total += get_exp(player.len() as i32, average_team_level, monster.clone());
    }
    player = calc_level_up(player, exp_total);
    (player, rewards)
}
