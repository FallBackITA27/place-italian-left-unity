use std::fmt::Display;

use image::{DynamicImage, ImageReader};

use crate::tile_coords::TileCoords;

pub struct MapCoords {
    lat: f64,
    lng: f64,
    zoom: f32,
}

impl MapCoords {
    pub fn get_lat(&self) -> f64 {
        self.lat
    }
    pub fn get_lng(&self) -> f64 {
        self.lng
    }
    pub fn get_zoom(&self) -> f32 {
        self.zoom
    }
    pub fn get_link(&self) -> String {
        format!(
            "https://wplace.live/?lat={}&lng={}&zoom={}",
            self.lat,
            self.lng,
            self.get_zoom()
        )
    }

    pub fn from_tile_coords(tile_coords: &TileCoords, width: u32) -> Self {
        let rel_x = ((tile_coords.get_tile_x() as f64) * 1000f64 + (tile_coords.get_x() as f64))
            / (2048f64 * 1000f64); // Relative X
        let rel_y = 1f64
            - ((tile_coords.get_tile_y() as f64) * 1000f64 + (tile_coords.get_y() as f64))
                / (2048f64 * 1000f64); // Relative Y
        Self {
            lat: 360f64
                * (std::f64::consts::E.powf((rel_y * 2f64 - 1f64) * std::f64::consts::PI)).atan()
                / std::f64::consts::PI
                - 90f64,
            lng: rel_x * 360f64 - 180f64,
            zoom: match width {
                0..1 => 22.0,
                1..10 => 20.0,
                10..60 => 17.0,
                60..100 => 14.0,
                100..300 => 13.0,
                300..500 => 12.5,
                500..1000 => 12.0,
                _ => 11.0,
            },
        }
    }
}

pub struct ImageInfo {
    file_name: String,
    image: image::DynamicImage,
    width: u32,
    height: u32,
}

impl ImageInfo {
    pub fn get_file_name(&self) -> &str {
        &self.file_name
    }

    pub fn get_image(&self) -> &DynamicImage {
        &self.image
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }
}

#[derive(Copy, Clone)]
pub enum Alliances {
    None,
    BrindisiPlace,
}

impl Alliances {
    fn to_markdown_alliance_list(self) -> String {
        format!(
            "  - [Progetti {}](#progetti-{})\n",
            self,
            self.to_string()
                .to_lowercase()
                .replace(" ", "-")
                .replace("(", "")
                .replace(")", "")
                .replace(",", "")
                .replace("!", "")
        )
    }

    pub fn markdown_list() -> String {
        [Self::BrindisiPlace]
            .into_iter()
            .map(|v| v.to_markdown_alliance_list())
            .collect()
    }
}

impl Display for Alliances {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::None => "Italian Left Unity",
                Self::BrindisiPlace => "BRINDISIPLACE",
            }
        )
    }
}

impl TryFrom<&str> for Alliances {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "-" => Ok(Self::None),
            "brindisiplace" => Ok(Self::BrindisiPlace),
            _ => Err(()),
        }
    }
}

pub struct ArtData {
    title: String,
    image_info: ImageInfo,
    tile_coords: TileCoords,
    map_coords: MapCoords,
    alliance: Alliances,
}

impl ArtData {
    pub fn new() -> Vec<Self> {
        let data = super::ART_FILE;

        let mut line_split = data.split('\n');
        let mut out = vec![];
        while let Some(_) = line_split.next() {
            // Split Line
            let title = line_split.next().expect("Wrong line number art data");
            let file_name = line_split.next().expect("Wrong line number art data");
            let tile_coords = line_split.next().expect("Wrong line number art data");
            let alliance = line_split.next().expect("Wrong line number art data");

            // Get Tile Coords
            let tile_coords = TileCoords::parse_tile_coords_string(tile_coords);

            // Get Image Path
            let path = String::from("../../templates/wplace/") + file_name;
            let path = std::path::Path::new(path.as_str());
            let path = std::path::absolute(path).expect("Couldn't make path absolute");
            if !path.exists() {
                panic!("Expected an existing image's path for image {file_name}.");
            }

            // Get Image
            let mut image = ImageReader::open(path).expect("Couldn't open image");
            image.set_format(image::ImageFormat::Png);
            let decoded_image = image.decode().expect("Couldn't decode image");

            // Get Image Info
            let image_info = ImageInfo {
                file_name: file_name.to_string(),
                width: decoded_image.width(),
                height: decoded_image.height(),
                image: decoded_image,
            };

            // Get Map Coords
            let center_coords = TileCoords::new(
                tile_coords.get_tile_x(),
                tile_coords.get_tile_y(),
                tile_coords.get_x() + (image_info.get_width() / 2) as u16,
                tile_coords.get_y() + (image_info.get_height() / 2) as u16,
            );

            out.push(Self {
                title: title.to_string(),
                map_coords: MapCoords::from_tile_coords(&center_coords, image_info.get_width()),
                image_info,
                tile_coords,
                alliance: Alliances::try_from(alliance).expect("Couldn't parse alliance"),
            });
        }
        out
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }
    pub fn get_image_info(&self) -> &ImageInfo {
        &self.image_info
    }

    pub fn get_tile_coords(&self) -> &TileCoords {
        &self.tile_coords
    }

    pub fn get_map_coords(&self) -> &MapCoords {
        &self.map_coords
    }

    pub fn get_alliance(&self) -> Alliances {
        self.alliance
    }
}
