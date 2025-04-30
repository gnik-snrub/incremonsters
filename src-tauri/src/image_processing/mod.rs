
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
            println!("Image: {:?}", image_data);
        }
        Err(e) => {println!("Error: Image not ok: {:?}", e)},
    }
}
