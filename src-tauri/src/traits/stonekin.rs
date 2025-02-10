#[derive(Clone, Copy)]
pub enum StonekinTrait {
    Slateblade,
    Pebblebound,
    Bolderfist,
    Mountainheart,
}

pub struct Trait {
    name: String,
    description: String,
    trigger: Trigger,
    #[serde(skip)]
    #[serde(default = "default_callback")]
    callback: CallbackFn,
}
impl MonsterTrait for StonekinTrait {
    fn get(&self) -> Trait {
        match self {
            StonekinTrait::Slateblade => {
                Trait {
                    name: "Cliff's Edge".to_string(),
                    description: "Defense stat (50%) provides extra damage on hit by reinforcing its powerful blows".to_string(),
                    trigger: Trigger::OnHit,
                    callback: cliffs_edge
                }
            }
            StonekinTrait::Pebblebound => {
                Trait {
                    name: "Quaking Dodge".to_string(),
                    description: "Quick footwork and powerful stomping allows it a chance to dodge attacks at higher defense, while weakening the defense of enemies".to_string(),
                    trigger: Trigger::OnDamage,
                    callback: quaking_dodge
                }
            }
            StonekinTrait::Bolderfist => {

            }
            StonekinTrait::Mountainheart => {

            }
        }
    }
}

fn cliffs_edge(
    self_value: Option<Monster>,
    opponent: Option<Monster>,
    allies: Option<Vec<Monster>>,
    enemies: Option<Vec<Monster>>,
    damage: Option<i32>
) -> (Option<Monster>, Option<Monster>, Option<Vec<Monster>>, Option<Vec<Monster>>, Option<i32>) {
    let unwrapped_damage = damage.unwrap();
    let unwrapped_self = self_value.unwrap();
    unwrapped_damage += (unwrapped_self.def * 0.5);

    (None, None, None, None, Some(unwrapped_damage))
}

fn quaking_dodge(
    self_value: Option<Monster>,
    opponent: Option<Monster>,
    allies: Option<Vec<Monster>>,
    enemies: Option<Vec<Monster>>,
    damage: Option<i32>
) -> (Option<Monster>, Option<Monster>, Option<Vec<Monster>>, Option<Vec<Monster>>, Option<i32>) {
    let unwrapped_self = self_value.unwrap();
    let unwrapped_opponent = opponent.unwrap();
    let unwrapped_enemies = enemies.unwrap();
    let unwrapped_damage = damage.unwrap();

    let def_cubed = unwrapped_self.atk.pow(3);
    let atk_cubed = unwrapped_opponent.atk.pow(3);

    let probability = def_cubed / (def_cubed + atk_cubed).clamp(0.1, 0.9);

    if thread_rng().gen_bool(probability) {
        unwrapped_damage = 0;
        for enemy in unwrapped_enemies {
            enemy.def -= (enemy.def * 0.05);
        }
    }

    (None, None, None, Some(unwrapped_enemies), Some(unwrapped_damage))
}
