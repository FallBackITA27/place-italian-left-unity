use image::{GenericImageView, ImageReader};
use wplace_common::{art_data::ArtData, color::Color};

const IMAGE_PATH_PREFIX: &'static str = "../../templates/wplace/";

pub struct GriefChecker {
    incorrect_px_count: u32,
    missing_time_hrs: f64,
}

impl GriefChecker {
    pub async fn check(template: &ArtData) -> Self {
        let path = IMAGE_PATH_PREFIX.to_string() + template.get_image_file_name();
        let path = std::path::Path::new(path.as_str());
        let path = std::path::absolute(path).expect("Couldn't make path absolute");
        if !path.exists() {
            panic!("Expected an existing image's path.");
        }

        let mut image = ImageReader::open(path).expect("Couldn't open image");
        image.set_format(image::ImageFormat::Png);
        let decoded_image = image.decode().expect("Couldn't decode image");

        let img_height = decoded_image.height();
        let img_width = decoded_image.width();

        /* Every art is at least 1 width and height.
         * You need to subtract 1 to avoid off-by-one errors on the fact that width/height are actually lengths.
         * A tile is 1000x1000 px, coords are 0-999
         */
        let tiles_width = 1 + ((img_width - 1 + (template.get_tile_coords_x() as u32)) / 1000);
        let tiles_height = 1 + ((img_height - 1 + (template.get_tile_coords_y() as u32)) / 1000);
        let mut tiles = Vec::with_capacity((tiles_width) as usize);

        for tile_x in template.get_tile_coords_tile_x()
            ..template.get_tile_coords_tile_x() + (tiles_width as u16)
        {
            tiles.push(Vec::with_capacity(tiles_height as usize));

            for tile_y in template.get_tile_coords_tile_y()
                ..template.get_tile_coords_tile_y() + (tiles_height as u16)
            {
                let url =
                    format!("https://backend.wplace.live/files/s0/tiles/{tile_x}/{tile_y}.png");
                let mut image = ImageReader::new(std::io::Cursor::new(
                    reqwest::get(&url)
                        .await
                        .expect("Couldn't load image in reqwest")
                        .bytes()
                        .await
                        .expect("Couldn't get request bytes"),
                ));

                image.set_format(image::ImageFormat::Png);
                let decoded_image = image.decode().expect("Couldn't decode tile");

                tiles.last_mut().unwrap().push(decoded_image);
            }
        }

        let mut incorrect_px_count = 0;

        for x in 0..img_width {
            for y in 0..img_height {
                let pixel = unsafe { decoded_image.unsafe_get_pixel(x, y) };

                if pixel.0[3] != 255 {
                    // Transparent means not visible in template
                    continue;
                }

                let template_color = match Color::try_from(pixel.0) {
                    Ok(v) => v,
                    Err(_) => {
                        println!(
                            "Pixel at {x} {y} is not the right color in image template, it is #{:X}{:X}{:X}",
                            pixel.0[0], pixel.0[1], pixel.0[2]
                        );
                        continue;
                    }
                };

                let tile_x = (x + (template.get_tile_coords_x() as u32)) / 1000;
                let tile_y = (y + (template.get_tile_coords_y() as u32)) / 1000;
                let tile_x_coord = (x + (template.get_tile_coords_x() as u32)) % 1000;
                let tile_y_coord = (y + (template.get_tile_coords_y() as u32)) % 1000;
                let tile = tiles[tile_x as usize]
                    .get(tile_y as usize)
                    .expect("Tile Y didn't exist in Tiles vec");
                let tile_pixel = unsafe { tile.unsafe_get_pixel(tile_x_coord, tile_y_coord) };

                match Color::try_from(tile_pixel.0) {
                    Ok(v) => {
                        if v != template_color {
                            incorrect_px_count += 1
                        }
                    }
                    Err(_) => {
                        incorrect_px_count += 1;
                    }
                };
            }
        }

        Self {
            incorrect_px_count,
            missing_time_hrs: ((incorrect_px_count * 30) as f64) / 3600.0,
        }
    }

    pub fn to_markdown_str(&self, art_data: &ArtData) -> String {
        if self.incorrect_px_count == 0 {
            format!("- {title} è OK", title = art_data.get_title(),)
        } else {
            format!(
                "- [{title}](https://wplace.live/?lat={lat}&lng={lng}&zoom={zoom}); Mancano {px}px, circa {hrs:.1} ore",
                title = art_data.get_title(),
                lat = art_data.get_map_coords_lat(),
                lng = art_data.get_map_coords_lng(),
                zoom = art_data.get_map_coords_zoom(),
                px = self.incorrect_px_count,
                hrs = self.missing_time_hrs
            )
        }
    }
}
