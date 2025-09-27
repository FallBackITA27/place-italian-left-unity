use std::{
    collections::HashMap,
    hash::Hash,
    io::{Read, Write},
};

use curl::easy::Handler;
use image::{DynamicImage, GenericImageView, ImageReader};
use wplace_common::{color::Color, tile_coords::TileCoords};

const IMAGE_PATH_PREFIX: &'static str = "../../templates/wplace/";

pub struct AreaData {
    x: u16,
    y: u16,
    tile_x: u16,
    tile_y: u16,
    width: u16,
    height: u16,
}

pub struct Inputs {
    download: bool,
    heatmap: bool,
    area: AreaData,
}

#[derive(PartialEq)]
enum AreaType {
    Points,
    Template,
}

impl Inputs {
    pub fn read(data: &str) -> Vec<Self> {
        let mut line_split = data.split('\n');
        let mut out = vec![];
        while let Some(_) = line_split.next() {
            let actions = line_split
                .next()
                .expect("Wrong line number areas")
                .to_lowercase();
            let top_left_corner = line_split.next().expect("Wrong line number areas");
            let extra_data = line_split.next().expect("Wrong line number areas");

            let top_left_corner = TileCoords::parse_tile_coords_string(top_left_corner);

            let area_type = if actions.contains('t') {
                AreaType::Template
            } else if actions.contains('p') {
                AreaType::Points
            } else {
                panic!("Non-existing area type command");
            };

            let download = actions.contains('d');
            let heatmap = actions.contains('h');

            let (width, height) = match area_type {
                AreaType::Points => {
                    let second_point = TileCoords::parse_tile_coords_string(extra_data);

                    if second_point.get_tile_x() < top_left_corner.get_tile_x()
                        || second_point.get_tile_y() < top_left_corner.get_tile_y()
                        || (second_point.get_tile_x() == top_left_corner.get_tile_x()
                            && second_point.get_x() < top_left_corner.get_x())
                        || (second_point.get_tile_y() == top_left_corner.get_tile_y()
                            && second_point.get_y() < top_left_corner.get_y())
                    {
                        panic!("Deformed data, second point not bottom right");
                    }

                    let height = 1000 * (second_point.get_tile_y() - top_left_corner.get_tile_y())
                        + second_point.get_y()
                        - top_left_corner.get_y()
                        + 1;
                    let width = 1000 * (second_point.get_tile_x() - top_left_corner.get_tile_x())
                        + second_point.get_x()
                        - top_left_corner.get_x()
                        + 1;

                    (width, height)
                }
                AreaType::Template => {
                    let file = wplace_common::art_data::ArtData::read(wplace_common::ART_FILE)
                        .into_iter()
                        .find(|r| {
                            r.get_tile_coords_tile_x() == top_left_corner.get_tile_x()
                                && r.get_tile_coords_tile_y() == top_left_corner.get_tile_y()
                                && r.get_tile_coords_x() == top_left_corner.get_x()
                                && r.get_tile_coords_y() == top_left_corner.get_y()
                        })
                        .expect("Template at coordinates doesn't exist!");
                    let file_name = file.get_image_file_name();

                    let path = IMAGE_PATH_PREFIX.to_string() + file_name;
                    let path = std::path::Path::new(path.as_str());
                    let path = std::path::absolute(path).expect("Couldn't make path absolute");
                    if !path.exists() {
                        panic!("Expected an existing image's path.");
                    }

                    let mut image = ImageReader::open(path).expect("Couldn't open image");
                    image.set_format(image::ImageFormat::Png);
                    let decoded_image = image.decode().expect("Couldn't decode image");

                    (decoded_image.width() as u16, decoded_image.height() as u16)
                }
            };

            out.push(Self {
                download,
                heatmap,
                area: AreaData {
                    tile_x: top_left_corner.get_tile_x(),
                    tile_y: top_left_corner.get_tile_y(),
                    x: top_left_corner.get_x(),
                    y: top_left_corner.get_y(),
                    width,
                    height,
                },
            });
        }
        out
    }
}

pub struct UserData {
    username: String,
    id: u64,
    alliance: Option<String>,
    discord: Option<String>,
}

#[derive(serde::Deserialize)]
struct PxRequestGet {
    #[serde(rename = "paintedBy")]
    painted_by: PxRequestGetUser,
}

#[derive(serde::Deserialize)]
struct PxRequestGetUser {
    id: u64,
    name: String,
    #[serde(rename = "allianceName")]
    alliance_name: String,
    discord: Option<String>,
}

struct Data(Vec<u8>);
impl Handler for Data {
    fn write(&mut self, data: &[u8]) -> Result<usize, curl::easy::WriteError> {
        self.0.extend_from_slice(data);
        Ok(data.len())
    }
}

impl UserData {
    pub fn to_markdown(&self) -> String {
        format!(
            "  * {} #{}{}{}",
            self.username,
            self.id,
            match &self.alliance {
                Some(v) => format!(" [{v}]"),
                None => String::new(),
            },
            match &self.discord {
                Some(v) => format!(" @{v}"),
                None => String::new(),
            }
        )
    }

    pub fn to_markdown_colors(&self, index: usize) -> String {
        self.to_markdown() + "; Color: " + ASSIGNED_COLORS[index].to_string().as_str()
    }
}

const ASSIGNED_COLORS: [Color; 20] = [
    Color::Red,
    Color::Green,
    Color::Blue,
    Color::Yellow,
    Color::Orange,
    Color::Purple,
    Color::DarkIndigo,
    Color::Pink,
    Color::LightPink,
    Color::Brown,
    Color::White,
    Color::Black,
    Color::DeepRed,
    Color::Cyan,
    Color::DarkPurple,
    Color::DarkTeal,
    Color::LightGreen,
    Color::Gold,
    Color::Gray,
    Color::DarkOlive,
];

pub struct MainLoop {
    out_file: std::fs::File,
    user_data: Vec<UserData>,
    iteration_number: u8,
    downloaded_tiles: HashMap<(u16, u16), DynamicImage>,
}

impl MainLoop {
    pub fn new(out_file: std::fs::File) -> Self {
        Self {
            out_file,
            iteration_number: 0,
            user_data: vec![],
            downloaded_tiles: HashMap::new(),
        }
    }

    pub fn reset_input_related_data(&mut self) {
        self.user_data = vec![];
    }

    pub async fn generate(&mut self, v: Inputs) {
        self.iteration_number += 1;
        self.out_file
            .write_all(format!("Input {}\n", self.iteration_number).as_bytes())
            .expect("Couldn't write to file");
        self.reset_input_related_data();

        println!(
            "Checking area of size {}x{}, {}px",
            v.area.width,
            v.area.height,
            v.area.width * v.area.height
        );

        if v.download {
            let mut new_image = image::RgbaImage::new(v.area.width as u32, v.area.height as u32);

            for tile_x in v.area.tile_x..=v.area.tile_x + ((v.area.x + v.area.width) / 1000) {
                for tile_y in v.area.tile_y..=v.area.tile_y + ((v.area.y + v.area.height) / 1000) {
                    if self.downloaded_tiles.contains_key(&(tile_x, tile_y)) {
                        println!("Continues?");
                        continue;
                    }

                    let url =
                        format!("https://backend.wplace.live/files/s0/tiles/{tile_x}/{tile_y}.png");

                    let mut curl_client = curl::easy::Easy2::new(Data(Vec::new()));
                    curl_client.url(&url).expect("Couldn't select url");
                    curl_client.perform().expect("Couldn't perform");

                    println!("{url}");

                    let tile_request = curl_client.get_ref();

                    let mut image = ImageReader::new(std::io::Cursor::new(tile_request.0.clone()));

                    image.set_format(image::ImageFormat::Png);
                    let decoded_image = image.decode().expect("Couldn't decode tile");

                    self.downloaded_tiles
                        .insert((tile_x, tile_y), decoded_image);
                }
            }
            for x in 0..v.area.width {
                for y in 0..v.area.height {
                    let x_in_tile = (x + v.area.x) % 1000;
                    let y_in_tile = (y + v.area.y) % 1000;
                    let tile_x = v.area.tile_x + ((x + v.area.x) / 1000);
                    let tile_y = v.area.tile_y + ((y + v.area.y) / 1000);

                    let tile = self
                        .downloaded_tiles
                        .get(&(tile_x, tile_y))
                        .expect("Somehow did not download tile");
                    let pixel =
                        unsafe { tile.unsafe_get_pixel(x_in_tile as u32, y_in_tile as u32) };
                    new_image.put_pixel(x as u32, y as u32, pixel);
                }
            }
            new_image
                .save_with_format(
                    format!("input_{}.png", self.iteration_number),
                    image::ImageFormat::Png,
                )
                .expect("Couldn't save download image");
        }

        let mut heatmap_image = match v.heatmap {
            true => Some(image::RgbaImage::new(
                v.area.width as u32,
                v.area.height as u32,
            )),
            false => None,
        };

        for x in v.area.x..v.area.x + v.area.width {
            for y in v.area.y..v.area.y + v.area.height {
                let x_in_tile = x % 1000;
                let y_in_tile = y % 1000;
                let tile_x = v.area.tile_x + (x / 1000);
                let tile_y = v.area.tile_y + (y / 1000);

                let url = format!(
                    "https://backend.wplace.live/s0/pixel/{tile_x}/{tile_y}?x={x_in_tile}&y={y_in_tile}"
                );
                println!("Checking {url}");

                let mut curl_client = curl::easy::Easy2::new(Data(Vec::new()));
                let mut headers = curl::easy::List::new();
                headers.append("User-Agent: Mozilla/5.0 (X11; Linux x86_64; rv:142.0) Gecko/20100101 Firefox/142.0").expect("Couldn't append header");
                curl_client
                    .http_headers(headers)
                    .expect("Couldn't append headers to client");
                curl_client.url(&url).expect("Couldn't select url");
                curl_client.perform().expect("Couldn't perform");

                let px_request = curl_client.get_ref();
                let data = serde_json::from_slice::<PxRequestGet>(px_request.0.as_slice())
                    .expect("Couldn't deserialize data")
                    .painted_by;

                std::thread::sleep(std::time::Duration::from_millis(250));

                if data.id == 0 {
                    continue;
                }

                let user_and_index = self
                    .user_data
                    .iter()
                    .enumerate()
                    .find(|d| d.1.id == data.id);
                let index = match user_and_index {
                    None => {
                        let user_data = UserData {
                            id: data.id,
                            username: data.name,
                            alliance: match data.alliance_name.is_empty() {
                                true => None,
                                false => Some(data.alliance_name),
                            },
                            discord: data.discord,
                        };
                        let index = self.user_data.len();
                        self.out_file
                            .write_all(
                                (match v.heatmap {
                                    false => user_data.to_markdown(),
                                    true => user_data.to_markdown_colors(index),
                                } + "\n")
                                    .as_bytes(),
                            )
                            .expect("Couldn't write user to out_file");
                        self.user_data.push(user_data);
                        index
                    }
                    Some((index, _)) => index,
                };
                if let Some(heatmap_image) = &mut heatmap_image {
                    heatmap_image.put_pixel(
                        (x - v.area.x) as u32,
                        (y - v.area.y) as u32,
                        image::Rgba::from(<[u8; 4]>::from(ASSIGNED_COLORS[index])),
                    );
                    heatmap_image
                        .save_with_format(
                            format!("heatmap_{}.png", self.iteration_number),
                            image::ImageFormat::Png,
                        )
                        .expect("Couldn't save download image");
                }
            }
        }
    }
}
