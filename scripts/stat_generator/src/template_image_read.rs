use image::{GenericImageView, ImageReader};

use crate::color::Color;

pub struct TemplateImageRead {
    total_px: i32,
    total_px_hrs: f64,
    px_counts: Vec<(Color, u64)>,
}

impl TemplateImageRead {
    pub fn image_calc(path: &str) -> Self {
        let path = std::path::Path::new(&path);

        println!("Provided the following path: {path:?}");

        let path = std::path::absolute(path).expect("Couldn't make path absolute");
        println!("Reading path as {path:?}");

        if !path.exists() {
            panic!("Expected an existing image's path.")
        }

        let mut image = ImageReader::open(path).expect("Couldn't open image");
        image.set_format(image::ImageFormat::Png);
        let decoded_image = image.decode().expect("Couldn't decode image");

        let img_height = decoded_image.height();
        let img_width = decoded_image.width();

        let mut hashmap: std::collections::HashMap<Color, u64> = std::collections::HashMap::new();
        let mut total = 0i32;

        for y in 0..img_height {
            for x in 0..img_width {
                let pixel = unsafe { decoded_image.unsafe_get_pixel(x, y) };

                if pixel.0[3] != 255 {
                    // Transparent means not visible in template
                    continue;
                }

                let color = match Color::try_from(pixel.0) {
                    Ok(v) => v,
                    Err(_) => {
                        println!(
                            "Pixel at {x} {y} is not the right color, it is #{:X}{:X}{:X}",
                            pixel.0[0], pixel.0[1], pixel.0[2]
                        );
                        continue;
                    }
                };

                match hashmap.get(&color) {
                    None => hashmap.insert(color, 1),
                    Some(num) => hashmap.insert(color, num + 1),
                };

                total += 1;
            }
        }

        let mut hashmap: Vec<(Color, u64)> = hashmap.iter().map(|(x, y)| (*x, *y)).collect();
        hashmap.sort_by(|(_, c1), (_, c2)| c2.cmp(c1));

        Self {
            total_px: total,
            total_px_hrs: (f64::from(total) / 2.0) / 60.0,
            px_counts: hashmap,
        }
    }

    pub fn to_markdown_str(self) -> String {
        format!(
            "- Lista Pixel: (Totale: {}, {:.1} ore)\n{}",
            self.total_px,
            self.total_px_hrs,
            self.px_counts
                .into_iter()
                .map(|(color, count)| { format!("  1. {}: {count}\n", color.to_string()) })
                .collect::<String>()
        )
    }
}
