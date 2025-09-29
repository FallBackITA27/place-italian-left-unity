use std::collections::HashMap;

use image::{DynamicImage, GenericImageView, ImageReader};
use wplace_common::{
    art_data::{Alliances, ArtData},
    color::Color,
    tile_coords::TileCoords,
};

pub struct GriefCheck {
    total_px_count: u32,
    incorrect_px_count: u32,
    missing_time_hrs: f64,
    wrong_px_coords: Vec<(TileCoords, Color)>,
    alliance: Alliances,
}

impl GriefCheck {
    pub fn get_incorrect_px_count(&self) -> u32 {
        self.incorrect_px_count
    }

    pub fn get_missing_time_hrs(&self) -> f64 {
        self.missing_time_hrs
    }

    pub fn get_total_px(&self) -> u32 {
        self.total_px_count
    }

    pub fn to_markdown_str(&self, art_data: &ArtData) -> String {
        format!(
            "- {title}; {px}px ≅ {hrs:.1}h\n-# [Mappa]({map_link}); [Template](https://github.com/FallBackITA27/place-italian-left-unity/blob/main/templates/wplace/{image_png}){alliance_data}",
            title = art_data.get_title(),
            map_link = art_data.get_map_coords().get_link(),
            px = self.incorrect_px_count,
            hrs = self.missing_time_hrs,
            image_png = art_data.get_image_info().get_file_name(),
            alliance_data = match self.alliance {
                Alliances::None => String::new(),
                v => format!("; [{v}]"),
            }
        )
    }

    pub fn print_wrong_px_coords(&self) -> String {
        self.wrong_px_coords
            .iter()
            .map(|v| {
                String::from("  * ")
                    + print_tile_coords(&v.0).as_str()
                    + " "
                    + v.1.to_string().as_str()
                    + "\n"
            })
            .collect::<String>()
    }
}

pub struct GriefChecker {
    loaded_tiles: HashMap<(u16, u16), DynamicImage>,
}

impl GriefChecker {
    pub fn new() -> Self {
        Self {
            loaded_tiles: HashMap::new(),
        }
    }

    pub async fn check(&mut self, template: &ArtData) -> GriefCheck {
        let image_info = template.get_image_info();

        let decoded_image = image_info.get_image();
        let img_height = image_info.get_height();
        let img_width = image_info.get_width();

        let tile_coords = template.get_tile_coords();

        /* Every art is at least 1 width and height.
         * You need to subtract 1 to avoid off-by-one errors on the fact that width/height are actually lengths.
         * A tile is 1000x1000 px, coords are 0-999
         */
        let tiles_width = 1 + ((img_width - 1 + (tile_coords.get_x() as u32)) / 1000);
        let tiles_height = 1 + ((img_height - 1 + (tile_coords.get_y() as u32)) / 1000);

        for tile_x in tile_coords.get_tile_x()..tile_coords.get_tile_x() + (tiles_width as u16) {
            for tile_y in tile_coords.get_tile_y()..tile_coords.get_tile_y() + (tiles_height as u16)
            {
                if self.loaded_tiles.contains_key(&(tile_x, tile_y)) {
                    continue;
                }

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

                self.loaded_tiles.insert((tile_x, tile_y), decoded_image);
            }
        }

        let mut incorrect_px_count = 0;
        let mut total_px_count = 0;
        let mut wrong_px_coords = vec![];

        for x in 0..img_width {
            for y in 0..img_height {
                let pixel = unsafe { decoded_image.unsafe_get_pixel(x, y) };

                if pixel.0[3] != 255 {
                    // Transparent means not visible in template
                    continue;
                }

                total_px_count += 1;

                let template_color = match Color::try_from(pixel.0) {
                    Ok(v) => v,
                    Err(_) => {
                        panic!(
                            "Pixel at {x} {y} is not the right color in image template, it is #{:X}{:X}{:X}",
                            pixel.0[0], pixel.0[1], pixel.0[2]
                        );
                    }
                };

                let tile_x = (x + (tile_coords.get_x() as u32)) / 1000;
                let tile_y = (y + (tile_coords.get_y() as u32)) / 1000;
                let tile_x_coord = ((x as u16) + tile_coords.get_x()) % 1000;
                let tile_y_coord = ((y as u16) + tile_coords.get_y()) % 1000;
                let tile = self
                    .loaded_tiles
                    .get(&(
                        (tile_x as u16) + tile_coords.get_tile_x(),
                        ((tile_y as u16) + tile_coords.get_tile_y()),
                    ))
                    .expect("Tile didn't exist in Loaded Tiles");
                let tile_pixel =
                    unsafe { tile.unsafe_get_pixel(tile_x_coord as u32, tile_y_coord as u32) };

                if let Ok(v) = Color::try_from(tile_pixel.0)
                    && v == template_color
                {
                    continue;
                }
                incorrect_px_count += 1;
                wrong_px_coords.push((
                    TileCoords::new(
                        (tile_x as u16) + tile_coords.get_tile_x(),
                        (tile_y as u16) + tile_coords.get_tile_y(),
                        tile_x_coord,
                        tile_y_coord,
                    ),
                    template_color,
                ));
            }
        }

        GriefCheck {
            total_px_count,
            incorrect_px_count,
            /* ((incorrect_px_count * 30) as f64) / 3600.0 */
            missing_time_hrs: (incorrect_px_count as f64) / 120.0,
            wrong_px_coords,
            alliance: template.get_alliance(),
        }
    }
}

fn print_tile_coords(v: &TileCoords) -> String {
    format!(
        "Tile X: {}, Tile Y: {}, X: {}, Y: {}",
        v.get_tile_x(),
        v.get_tile_y(),
        v.get_x(),
        v.get_y()
    )
}
