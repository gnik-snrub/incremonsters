use std::collections::HashSet;

use image::{DynamicImage, GenericImageView, Rgba, RgbaImage};
use rand::{Rng, SeedableRng};

use crate::models::Monster;


#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct RGBAValue {
    r: u8,
    g: u8,
    b: u8,
    a: u8
}

#[derive(Debug)]
struct ImageData {
    width: usize,
    height: usize,
    pixels: Vec<RGBAValue>
}

impl ImageData {
    fn get_pixel(&self, x: usize, y: usize) -> &RGBAValue {
        let pixel = &self.pixels[y * self.width + x];
        pixel
    }
}

#[tauri::command]
pub fn generate_fusion_sprite(import_palette: Monster, import_sprite: Monster) {
    let palette_monster = image::open(format!("../public/images/monsters/{}/{}.png", import_palette.creature_family, import_palette.creature_type));
    let sprite_monster = image::open(format!("../public/images/monsters/{}/{}.png", import_sprite.creature_family, import_sprite.creature_type));
    match (palette_monster, sprite_monster) {
        (Ok(colors), Ok(shape)) => {
            let (palette, _) = collect_palette_and_assignments(&colors);
            let (_, sprite) = collect_palette_and_assignments(&shape);

            create_sprite(palette, sprite, &shape, format!("{}-{}", import_palette.creature_type, import_sprite.creature_type).as_str());
        }
        (_, Err(e)) | (Err(e), _) => {println!("Error: Image not ok: {:?}", e)},
    }
}

fn collect_palette_and_assignments(img: &DynamicImage) -> (Vec<RGBAValue>, Vec<isize>) {
    let buffer = img.to_rgba8();
    let raw = buffer.as_raw();
    let height = buffer.height();
    let width = buffer.width();

    let mut image_data = ImageData {
        height: height as usize,
        width: width as usize,
        pixels: vec![]
    };

    for chunk in raw.chunks(4) {
        let new_pixel = RGBAValue {
            r: chunk[0],
            g: chunk[1],
            b: chunk[2],
            a: chunk[3],
        };
        image_data.pixels.push(new_pixel);
    }

    let mut centroids = initialize_centroids(&image_data, 25);
    let mut assignments = centroid_distance_assigning(&centroids, &image_data);
    let mut reinit_count: usize = 0;
    for i in 0..10 {
        let new_centroids = recompute_centroids(&centroids, &image_data, &assignments, reinit_count);
        let change = sum_centroid_distances(&centroids, &new_centroids);
        centroids = new_centroids;
        if change.abs() <= 50 {
            break;
        }
        assignments = centroid_distance_assigning(&centroids, &image_data);
    }
    (centroids, assignments)
}

fn sum_centroid_distances(old: &Vec<RGBAValue>, new: &Vec<RGBAValue>) -> i32 {
    old.iter().zip(new.iter())
        .map(|(a, b)| {
            let dr = a.r as i32 - b.r as i32;
            let dg = a.g as i32 - b.g as i32;
            let db = a.b as i32 - b.b as i32;
            let distance: i32 = dr * dr + dg * dg + db * db;
            distance
        })
        .sum()
}

fn initialize_centroids(image: &ImageData, k: i32) -> Vec<RGBAValue> {
    let mut centroids = vec![];
    for _ in 0..k {
        centroids.push(get_init_pixel(image, centroids.len()));
    }

    centroids
}

fn get_init_pixel(image: &ImageData, init_count: usize) -> RGBAValue {
    let mut rng = rand::rngs::StdRng::seed_from_u64(31 + init_count as u64);

    let mut sampled_pixels: HashSet<(usize, usize)> = HashSet::new();
    let max_samples = 20;

    let mut rand_x = rng.gen_range(0..image.width);
    let mut rand_y = rng.gen_range(0..image.height);
    let mut sample_attempts = 1;
    let mut pixel = image.get_pixel(rand_x, rand_y);

    loop {
        if image.get_pixel(rand_x, rand_y).a == 0 {
            rand_x = rng.gen_range(0..image.width);
            rand_y = rng.gen_range(0..image.height);
            pixel = image.get_pixel(rand_x, rand_y);
            sample_attempts += 1;
            continue;
        }
        if !sampled_pixels.contains(&(rand_x, rand_y)) {
            sampled_pixels.insert((rand_x, rand_y));
            break;
        }
        if sample_attempts >= max_samples {
            break;
        }
        rand_x = rng.gen_range(0..image.width);
        rand_y = rng.gen_range(0..image.height);
        pixel = image.get_pixel(rand_x, rand_y);
        sample_attempts += 1;
    }

    let centroid = RGBAValue {
        r: pixel.r,
        g: pixel.g,
        b: pixel.b,
        a: 255,
    };

    centroid
}

fn centroid_distance_assigning(centroids: &Vec<RGBAValue>, image: &ImageData) -> Vec<isize> {
    let mut centroid_assignments = vec![];

    for pixel in &image.pixels {
        if pixel.a == 0 {
            centroid_assignments.push(-1);
            continue;
        }

        let mut nearest: (isize, i32) = (-1, -1);

        for (i, centroid) in centroids.iter().enumerate() {
            let dr = pixel.r as i32 - centroid.r as i32;
            let dg = pixel.g as i32 - centroid.g as i32;
            let db = pixel.b as i32 - centroid.b as i32;
            let distance: i32 = dr * dr + dg * dg + db * db;

            if nearest.1 == -1 || distance < nearest.1 {
                nearest = (i as isize, distance);
            }
        }

        centroid_assignments.push(nearest.0);
    }

    centroid_assignments
}

#[derive(Clone)]
struct RecomputeData {
    r: i32,
    g: i32,
    b: i32,
    count: i32,
}

fn recompute_centroids(centroids: &Vec<RGBAValue>, image: &ImageData, assignments: &Vec<isize>, mut reinit_count: usize) -> Vec<RGBAValue> {
    let default_recompute_data = RecomputeData { r: 0, g: 0, b: 0, count: 0 };
    let mut recompute_data: Vec<RecomputeData> = vec![default_recompute_data.clone(); centroids.len()];

    for (image_idx, pixel) in assignments.iter().enumerate() {
        if *pixel == -1 {
            continue;
        }
        recompute_data[*pixel as usize].r += image.pixels[image_idx].r as i32;
        recompute_data[*pixel as usize].g += image.pixels[image_idx].g as i32;
        recompute_data[*pixel as usize].b += image.pixels[image_idx].b as i32;
        recompute_data[*pixel as usize].count += 1;
    }

    let mut unique_centroids: HashSet<RGBAValue> = HashSet::new();

    let mut recomputed_centroids = vec![];
    for d in recompute_data {
        if d.count == 0 {
            recomputed_centroids.push(get_init_pixel(image, reinit_count));
            reinit_count += 1;
            continue;
        }

        let updated_centroid = RGBAValue {
            r: (d.r / if d.count == 0 { 1 } else { d.count }) as u8,
            g: (d.g / if d.count == 0 { 1 } else { d.count }) as u8,
            b: (d.b / if d.count == 0 { 1 } else { d.count }) as u8,
            a: 255,
        };

        if unique_centroids.contains(&updated_centroid) {
            recomputed_centroids.push(get_init_pixel(image, reinit_count));
            reinit_count += 1;
            continue;
        }
        unique_centroids.insert(updated_centroid.clone());
        recomputed_centroids.push(updated_centroid);
    }

    recomputed_centroids
}

fn create_sprite(palette: Vec<RGBAValue>, sprite: Vec<isize>, original_image: &DynamicImage, file_name: &str) {
    let w = original_image.width() as u32;
    let h = original_image.height() as u32;
    let mut img = RgbaImage::new(w, h);

    for (i, c) in sprite.iter().enumerate() {
        let x = (i as u32) % w;
        let y = (i as u32) / w;

        if *c == -1 {
            img.put_pixel(x, y, Rgba([0, 0, 0, 0]));
            continue;
        }

        let assigned_color = &palette[*c as usize];
        let alpha_value = original_image.get_pixel(x, y)[3];
        img.put_pixel(x, y, Rgba([assigned_color.r, assigned_color.g, assigned_color.b, alpha_value]));
    }

    let path_name = format!("../public/images/{}.png", file_name);

    img.save(path_name).expect("Failed to save image");
}
