use std::collections::HashSet;

use rand::Rng;


#[derive(Debug)]
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
pub fn collect_image_data() {
    let image_file = image::open("../public/images/test.png");
    match image_file {
        Ok(img) => {
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
            let centroids = initialize_centroids(&image_data, 30);
            let assigned = centroid_distance_assigning(centroids, &image_data);
            println!("Assigned pixels: {:?}", assigned);
        }
        Err(e) => {println!("Error: Image not ok: {:?}", e)},
    }
}

fn initialize_centroids(image: &ImageData, k: i32) -> Vec<RGBAValue> {
    let mut rng = rand::thread_rng();
    let mut sampled_pixels: HashSet<(usize, usize)> = HashSet::new();
    let mut centroids = vec![];
    let max_samples = 20;

    for _ in 0..k {
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

        centroids.push(centroid);
    }


    centroids
}

fn centroid_distance_assigning(centroids: Vec<RGBAValue>, image: &ImageData) -> Vec<isize> {
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
