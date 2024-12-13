pub fn damage_calculation(atk: i32, def: i32) -> i32 {
    let calculated: f32 = atk as f32 * ((atk as f32) / if (def) <= 0 { atk } else { atk + def } as f32);
    calculated.round() as i32
}
