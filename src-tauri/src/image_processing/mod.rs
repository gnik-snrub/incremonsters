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
            let centroids = initialize_centroids(image_data, 30);
        }
        Err(e) => {println!("Error: Image not ok: {:?}", e)},
    }
}

fn initialize_centroids(image: ImageData, k: i32) -> Vec<RGBAValue> {
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
