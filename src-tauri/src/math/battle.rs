use crate::models::{ModMode, ModType, ModifierKind, Monster, StatAdjustments, StatusEffect, TemporaryModifier, Trigger};
use serde::{Deserialize, Serialize};
use rand::seq::SliceRandom;
use rand::random;

fn get_speed_order(player: &Vec<Monster>, enemy: &Vec<Monster>, player_speed_mod: f32) -> Vec<(i32, String, usize)> {
    let mut combined: Vec<(i32, String, usize)> = Vec::new();
    for (i, monster) in player.iter().enumerate() {
        combined.push((((monster.spd + monster.stat_adjustments.spd) as f32 * player_speed_mod) as i32, "player".to_string(), i));
    }
    for (i, monster) in enemy.iter().enumerate() {
        combined.push((monster.spd + monster.stat_adjustments.spd, "enemy".to_string(), i));
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

fn get_target(monsters: &Vec<Monster>) -> Option<usize> {
    let mut rng = rand::thread_rng();
    let mut alive: Vec<usize> = monsters
        .iter()
        .enumerate()
        .filter_map(|(idx, m)| {
            if m.damage < m.hp + m.stat_adjustments.hp {
                Some(idx)
            } else {
                None
            }
        }).collect::<Vec<_>>();

    alive.shuffle(&mut rng);
    if alive.is_empty() {
        return None;
    }
    return Some(alive[0]);
}

pub fn damage_calculation(atk: i32, def: i32) -> i32 {
    if atk <= 0 {
        return 0;
    }

    let f32_atk: f32 = atk as f32;
    let f32_def: f32 = def as f32;

    let divisor: f32 = if def <= 0 {
        f32_atk
    } else {
        f32_atk + f32_def
    };
    let calculated: f32 = f32_atk * (f32_atk / divisor);
    calculated.ceil() as i32
}

#[derive(Serialize, Deserialize)]
pub struct GlobalModifier {
    source_id: String,
    name: String,
    description: String,
    mod_mode: ModMode,
    mod_value: f32,
}

#[derive(Serialize, Deserialize)]
pub struct GlobalModifiers {
    atk: Vec<GlobalModifier>,
    def: Vec<GlobalModifier>,
    spd: Vec<GlobalModifier>,
    hp: Vec<GlobalModifier>,
}

fn calculate_modifier(mods: Vec<GlobalModifier>) -> f32 {
    let mut mod_total: f32 = 1.0;
    for m in mods {
        match m.mod_mode {
            ModMode::Add => {
                mod_total += m.mod_value;
            },
            ModMode::Sub => {
                mod_total -= m.mod_value;
            },
            ModMode::Div => {
                mod_total /= m.mod_value;
            },
            ModMode::Mult => {
                mod_total *= m.mod_value;
            },
        }
    }
    mod_total
}


#[tauri::command]
pub fn battle(mut player: Vec<Monster>, mut enemy: Vec<Monster>, global_modifiers: GlobalModifiers, mut logs: Vec<String>) -> ([Vec<Monster>; 2], Vec<String>) {
    let mut logs = Vec::new();
    let attack_mod = calculate_modifier(global_modifiers.atk);
    let defense_mod = calculate_modifier(global_modifiers.def);
    let speed_mod = calculate_modifier(global_modifiers.spd);
    let hp_mod = calculate_modifier(global_modifiers.hp);
    let ordered: Vec<(i32, String, usize)> = get_speed_order(&player, &enemy, speed_mod);
    for (_, side, index) in ordered {
        if side == "player" {
            process_status_effects(&mut player[index], &mut logs);
            if is_stunned(&player[index]) || is_frozen(&player[index]) {
                continue;
            }

            player[index].stat_adjustments = adjust_for_temporary_modifiers(&player[index]);
            match get_target(&enemy) {
                Some(target_idx) => {
                    enemy[target_idx].stat_adjustments = adjust_for_temporary_modifiers(&enemy[target_idx]);
                    // OnAttack trait triggers
                    let (attack_trigger_self,
                        attack_trigger_opponent,
                        attack_trigger_player,
                        attack_trigger_enemy,
                        damage_not_yet_created) = player[index].trigger_traits(Trigger::OnAttack, &Some(enemy[target_idx].clone()), Some(player.clone()), Some(enemy.clone()), None, &mut logs);
                    let attack_tuple = unwrap_trigger_options((player[index].clone(), enemy[target_idx].clone(), player.clone(), enemy.clone(), 0),
                        (attack_trigger_self, attack_trigger_opponent, attack_trigger_player, attack_trigger_enemy, damage_not_yet_created));
                    player[index] = attack_tuple.0;
                    enemy[target_idx] = attack_tuple.1;
                    player = attack_tuple.2;
                    enemy = attack_tuple.3;

                    if is_blind(&player[index]) && rand::random::<bool>() {
                        logs.push(format!("{} is blinded and misses the attack!", player[index].name));
                        if enemy[target_idx].damage > enemy[target_idx].hp + enemy[target_idx].stat_adjustments.hp {
                            enemy[target_idx].damage = enemy[target_idx].hp;
                        }
                        continue;
                    }

                    // Damage calculation
                    let mut damage: i32 = damage_calculation(((player[index].atk + player[index].stat_adjustments.atk) as f32 * attack_mod) as i32,
                        enemy[target_idx].def + enemy[target_idx].stat_adjustments.def);

                    // OnHit trait triggers
                    let (hit_trigger_self,
                        hit_trigger_opponent,
                        hit_trigger_player,
                        hit_trigger_enemy,
                        hit_trigger_damage) = player[index].trigger_traits(Trigger::OnHit, &Some(enemy[target_idx].clone()), Some(player.clone()), Some(enemy.clone()), Some(damage), &mut logs);
                    let hit_tuple = unwrap_trigger_options((player[index].clone(), enemy[target_idx].clone(), player.clone(), enemy.clone(), damage),
                        (hit_trigger_self, hit_trigger_opponent, hit_trigger_player, hit_trigger_enemy, hit_trigger_damage));
                    player[index] = hit_tuple.0;
                    enemy[target_idx] = hit_tuple.1;
                    player = hit_tuple.2;
                    enemy = hit_tuple.3;
                    damage = hit_tuple.4;

                    // OnDefend trait triggers
                    let (defend_trigger_self,
                        defend_trigger_opponent,
                        defend_trigger_player,
                        defend_trigger_enemy,
                        defend_trigger_damage) = player[index].trigger_traits(Trigger::OnDefend, &Some(enemy[target_idx].clone()), Some(player.clone()), Some(enemy.clone()), Some(damage), &mut logs);
                    let defend_tuple = unwrap_trigger_options((player[index].clone(), enemy[target_idx].clone(), player.clone(), enemy.clone(), damage),
                        (defend_trigger_self, defend_trigger_opponent, defend_trigger_player, defend_trigger_enemy, defend_trigger_damage));
                    player[index] = defend_tuple.0;
                    enemy[target_idx] = defend_tuple.1;
                    player = defend_tuple.2;
                    enemy = defend_tuple.3;
                    damage = defend_tuple.4;

                    // Damage is applied
                    enemy[target_idx].damage += damage;

                    // OnDamage trait triggers
                    let (damage_trigger_self,
                        damage_trigger_opponent,
                        damage_trigger_player,
                        damage_trigger_enemy,
                        damage_trigger_damage) = player[index].trigger_traits(Trigger::OnDamage, &Some(enemy[target_idx].clone()), Some(player.clone()), Some(enemy.clone()), Some(damage), &mut logs);
                    let damage_tuple = unwrap_trigger_options((player[index].clone(), enemy[target_idx].clone(), player.clone(), enemy.clone(), damage),
                        (damage_trigger_self, damage_trigger_opponent, damage_trigger_player, damage_trigger_enemy, damage_trigger_damage));
                    player[index] = damage_tuple.0;
                    enemy[target_idx] = damage_tuple.1;
                    player = damage_tuple.2;
                    enemy = damage_tuple.3;
                    _ = damage_tuple.4;

                    // Damage is capped to HP, so as to not ip into negative health
                    if enemy[target_idx].damage > enemy[target_idx].hp + enemy[target_idx].stat_adjustments.hp {
                        enemy[target_idx].damage = enemy[target_idx].hp;
                    }
                }
                None => continue,
            }
        } else {
            process_status_effects(&mut enemy[index], &mut logs);
            if is_stunned(&player[index]) || is_frozen(&enemy[index]) {
                continue;
            }

            match get_target(&player) {
                Some(target_idx) => {
                    // OnAttack trait triggers
                    let (attack_trigger_self,
                        attack_trigger_opponent,
                        attack_trigger_player,
                        attack_trigger_enemy,
                        damage_not_yet_created) = enemy[index].trigger_traits(Trigger::OnAttack, &Some(player[target_idx].clone()), Some(enemy.clone()), Some(player.clone()), None, &mut logs);
                    let attack_tuple = unwrap_trigger_options((enemy[index].clone(), player[target_idx].clone(), enemy.clone(), player.clone(), 0),
                        (attack_trigger_self, attack_trigger_opponent, attack_trigger_player, attack_trigger_enemy, damage_not_yet_created));
                    enemy[index] = attack_tuple.0;
                    player[target_idx] = attack_tuple.1;
                    enemy = attack_tuple.2;
                    player = attack_tuple.3;

                    if is_blind(&player[index]) && rand::random::<bool>() {
                        logs.push(format!("{} is blinded and misses the attack!", enemy[index].name));
                        if player[target_idx].damage as f32 > (player[target_idx].hp + player[target_idx].stat_adjustments.hp) as f32 * hp_mod {
                            player[target_idx].damage = player[target_idx].hp;
                        }
                        continue;
                    }

                    // Damage calculation
                    let mut damage: i32 = damage_calculation(enemy[index].atk + enemy[index].stat_adjustments.atk,
                        ((player[target_idx].def + player[target_idx].stat_adjustments.def) as f32 * defense_mod) as i32);

                    // OnHit trait triggers
                    let (hit_trigger_self,
                        hit_trigger_opponent,
                        hit_trigger_player,
                        hit_trigger_enemy,
                        hit_trigger_damage) = enemy[index].trigger_traits(Trigger::OnAttack, &Some(player[target_idx].clone()), Some(enemy.clone()), Some(player.clone()), None, &mut logs);
                    let hit_tuple = unwrap_trigger_options((enemy[index].clone(), player[target_idx].clone(), enemy.clone(), player.clone(), damage),
                        (hit_trigger_self, hit_trigger_opponent, hit_trigger_player, hit_trigger_enemy, hit_trigger_damage));
                    enemy[index] = hit_tuple.0;
                    player[target_idx] = hit_tuple.1;
                    enemy = hit_tuple.2;
                    player = hit_tuple.3;
                    damage = hit_tuple.4;

                    // OnDefend trait triggers
                    let (defend_trigger_self,
                        defend_trigger_opponent,
                        defend_trigger_player,
                        defend_trigger_enemy,
                        defend_trigger_damage) = enemy[index].trigger_traits(Trigger::OnAttack, &Some(player[target_idx].clone()), Some(enemy.clone()), Some(player.clone()), None, &mut logs);
                    let defend_tuple = unwrap_trigger_options((enemy[index].clone(), player[target_idx].clone(), enemy.clone(), player.clone(), damage),
                        (defend_trigger_self, defend_trigger_opponent, defend_trigger_player, defend_trigger_enemy, defend_trigger_damage));
                    enemy[index] = defend_tuple.0;
                    player[target_idx] = defend_tuple.1;
                    enemy = defend_tuple.2;
                    player = defend_tuple.3;
                    damage = defend_tuple.4;

                    // Damage is applied
                    player[target_idx].damage += damage;
                    
                    // OnDamage trait triggers
                    let (damage_trigger_self,
                        damage_trigger_opponent,
                        damage_trigger_player,
                        damage_trigger_enemy,
                        damage_trigger_damage) = enemy[index].trigger_traits(Trigger::OnAttack, &Some(player[target_idx].clone()), Some(enemy.clone()), Some(player.clone()), None, &mut logs);
                    let damage_tuple = unwrap_trigger_options((enemy[index].clone(), player[target_idx].clone(), enemy.clone(), player.clone(), damage),
                        (damage_trigger_self, damage_trigger_opponent, damage_trigger_player, damage_trigger_enemy, damage_trigger_damage));
                    enemy[index] = damage_tuple.0;
                    player[target_idx] = damage_tuple.1;
                    enemy = damage_tuple.2;
                    player = damage_tuple.3;
                    _ = damage_tuple.4;

                    // Damage is capped to HP, so as to not ip into negative health
                    if player[target_idx].damage as f32 > (player[target_idx].hp + player[target_idx].stat_adjustments.hp) as f32 * hp_mod {
                        player[target_idx].damage = player[target_idx].hp;
                    }
                }
                None => continue,
            }
        }
    }
    let return_logs = logs.clone();
    ([player, enemy], return_logs)
}

fn unwrap_trigger_options(mut original: (Monster, Monster, Vec<Monster>, Vec<Monster>, i32),
    wrapped: (Option<Monster>, Option<Monster>, Option<Vec<Monster>>, Option<Vec<Monster>>, Option<i32>)) -> 
    (Monster, Monster, Vec<Monster>, Vec<Monster>, i32) {
    if wrapped.0.is_some() { original.0 = wrapped.0.unwrap(); }
    if wrapped.1.is_some() { original.1 = wrapped.1.unwrap(); }
    if wrapped.2.is_some() { original.2 = wrapped.2.unwrap(); }
    if wrapped.3.is_some() { original.3 = wrapped.3.unwrap(); }
    if wrapped.4.is_some() { original.4 = wrapped.4.unwrap(); }

    original
}

fn adjust_for_temporary_modifiers(monster: &Monster) -> StatAdjustments {
    let mut hp = 0;
    let mut atk = 0;
    let mut def = 0;
    let mut spd = 0;
    let mut dmg = 0;
    for t in &monster.temporary_modifiers {
        match (&t.mod_type, t.mod_value, &t.mod_mode) {
            (&Some(ModType::HP), Some(value), Some(mode)) => {
                hp = adjust_by_type(hp, value, mode);
            },
            (&Some(ModType::ATK), Some(value), Some(mode)) => {
                atk = adjust_by_type(hp, value, mode);
            },
            (&Some(ModType::DEF), Some(value), Some(mode)) => {
                def = adjust_by_type(def, value, mode);
            },
            (&Some(ModType::SPD), Some(value), Some(mode)) => {
                spd = adjust_by_type(spd, value, mode);
            },
            (&Some(ModType::DMG), Some(value), Some(mode)) => {
                dmg = adjust_by_type(dmg, value, mode);
            },
            _ => {},
        }
    }
    StatAdjustments {
        hp, atk, def, spd, dmg
    }
}

fn adjust_by_type(stat: i32, value: i32, mod_mode: &ModMode) -> i32 {
    match mod_mode {
        ModMode::Add => {
            stat + value
        },
        ModMode::Sub => {
            stat - value
        },
        ModMode::Div => {
            stat / value
        },
        ModMode::Mult => {
            stat * value
        },
    }
}

pub fn process_status_effects(monster: &mut Monster, logs: &mut Vec<String>) {
    let mut to_remove = Vec::new();
    let mut to_add = Vec::new();

    for (i, modifier) in monster.temporary_modifiers.iter_mut().enumerate() {
        if let ModifierKind::Status(effect) = &modifier.kind {
            match effect {
                StatusEffect::Poison => {
                    let dmg = (monster.hp as f32 * 0.05).ceil() as i32;
                    monster.damage += dmg;
                    logs.push(format!("{} takes {} poison damage!", monster.name, dmg));
                },
                StatusEffect::Burn => {
                    let dmg = (monster.hp as f32 * 0.03).ceil() as i32;
                    monster.damage += dmg;
                    logs.push(format!("{} is burned for {} damage!", monster.name, dmg));

                    to_add.push(TemporaryModifier {
                        source: "burn_debuff".to_string(),
                        kind: ModifierKind::Stat,
                        mod_type: Some(ModType::ATK),
                        mod_mode: Some(ModMode::Sub),
                        mod_value: Some((monster.atk as f32 * 0.1).ceil() as i32),
                        quantity: 1,
                    });
                }
                StatusEffect::Freeze => {
                    logs.push(format!("{} is frozen solid!", monster.name));

                    to_add.push(TemporaryModifier {
                        source: "burn_debuff".to_string(),
                        kind: ModifierKind::Stat,
                        mod_type: Some(ModType::SPD),
                        mod_mode: Some(ModMode::Sub),
                        mod_value: Some((monster.spd as f32 * 0.1).ceil() as i32),
                        quantity: 1,
                    });
                }
                StatusEffect::Stun => {
                    logs.push(format!("{} is stunned, and can't act this turn!", monster.name));
                }
                StatusEffect::Blind => {
                    logs.push(format!("{} is blinded, and may miss!", monster.name));
                }
            }

            modifier.quantity -= 1;
            if modifier.quantity <= 0 {
                to_remove.push(i);
                logs.push(format!("{}'s {:?} has worn off.", monster.name, effect));
            }
        }

    }

    for modifier in to_add.into_iter() {
        monster.temporary_modifiers.push(modifier);
    }

    for i in to_remove.iter().rev() {
        monster.temporary_modifiers.remove(*i);
    }

}

pub fn has_status(monster: &Monster, target: StatusEffect) -> bool {
    monster.temporary_modifiers.iter().any(|m| {
        matches!(&m.kind, ModifierKind::Status(e) if e == &target)
    })
}

pub fn is_stunned(monster:&Monster) -> bool {
    has_status(monster, StatusEffect::Stun)
}

pub fn is_frozen(monster:&Monster) -> bool {
    has_status(monster, StatusEffect::Freeze)
}

pub fn is_blind(monster:&Monster) -> bool {
    has_status(monster, StatusEffect::Blind)
}
