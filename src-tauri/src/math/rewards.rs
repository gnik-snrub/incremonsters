use crate::models::Monster;
use rand::Rng;

const REWARD_BASE: f32 = 10.0;
const REWARD_GROWTH: f32 = 1.07;

fn get_rewards(dungeon_lvl: i32, slain: &Vec<Monster>) -> i32 {
    let mut rng = rand::thread_rng();
    let mut reward_total: f32 = 0.0;
    for monster in slain.iter() {
        let variance: f32 = rng.gen_range(0.8..1.2);
        reward_total += (REWARD_BASE * REWARD_GROWTH.powf(monster.lvl as f32)) * variance * dungeon_lvl as f32;
    }
    reward_total.ceil() as i32
}
