pub struct TileCoords {
    tile_x: u16,
    tile_y: u16,
    x: u16,
    y: u16,
}

impl TileCoords {
    fn get_x(&self) -> u16 {
        self.x
    }
    fn get_y(&self) -> u16 {
        self.y
    }
    fn get_tile_x(&self) -> u16 {
        self.tile_x
    }
    fn get_tile_y(&self) -> u16 {
        self.tile_y
    }
}

pub struct MapCoords {
    lat: f64,
    lng: f64,
    zoom: f32,
}

impl MapCoords {
    fn get_lat(&self) -> f64 {
        self.lat
    }
    fn get_lng(&self) -> f64 {
        self.lng
    }
    fn get_zoom(&self) -> f32 {
        self.zoom
    }
}

pub struct ArtData {
    title: String,
    image_file_name: String,
    tile_coords: TileCoords,
    map_coords: MapCoords,
}

impl ArtData {
    pub fn read(data: &str) -> Vec<Self> {
        let mut line_split = data.split('\n');
        let mut out = vec![];
        while let Some(_) = line_split.next() {
            let title = line_split.next().expect("Wrong line number art data");
            let img_path = line_split.next().expect("Wrong line number art data");
            let tile_coords = line_split.next().expect("Wrong line number art data");
            let map_coords = line_split.next().expect("Wrong line number art data");

            let mut tile_coords = tile_coords
                .strip_prefix('(')
                .expect("Couldn't strip prefix tile coords")
                .strip_suffix(')')
                .expect("Couldn't strip suffix tile coords")
                .split(',')
                .map(|x| {
                    x.split(':')
                        .last()
                        .expect("Deformed tile coords data")
                        .trim()
                        .parse::<u16>()
                        .expect("No number parsing tile coords")
                });

            let mut lat = None;
            let mut lng = None;
            let mut zoom = None;
            for mut data in map_coords.split('&').map(|x| x.split('=')) {
                let k = data.next().expect("Deformed latlonzoom data");
                let v = data.next().expect("Deformed latlonzoom data");

                match k {
                    "lat" => lat = Some(v.parse::<f64>().expect("Couldn't parse f64 lat")),
                    "lng" => lng = Some(v.parse::<f64>().expect("Couldn't parse f64 lon")),
                    "zoom" => zoom = Some(v.parse::<f32>().expect("Couldn't parse f32 zoom")),
                    _ => panic!("Deformed split data has invalid key"),
                }
            }
            if lat.is_none() || lng.is_none() || zoom.is_none() {
                panic!("Coords split deformed data");
            }

            out.push(Self {
                title: title.to_string(),
                image_file_name: img_path.to_string(),
                tile_coords: TileCoords {
                    tile_x: tile_coords.next().expect("No Tile X coords found"),
                    tile_y: tile_coords.next().expect("No Tile Y coords found"),
                    x: tile_coords.next().expect("No X coords found"),
                    y: tile_coords.next().expect("No Y coords found"),
                },
                map_coords: MapCoords {
                    lat: lat.unwrap(),
                    lng: lng.unwrap(),
                    zoom: zoom.unwrap(),
                },
            });
        }
        out
    }

    pub fn get_title<'a>(&'a self) -> &'a str {
        &self.title
    }
    pub fn get_image_file_name<'a>(&'a self) -> &'a str {
        &self.image_file_name
    }

    pub fn get_tile_coords_x(&self) -> u16 {
        self.tile_coords.get_x()
    }
    pub fn get_tile_coords_y(&self) -> u16 {
        self.tile_coords.get_y()
    }
    pub fn get_tile_coords_tile_x(&self) -> u16 {
        self.tile_coords.get_tile_x()
    }
    pub fn get_tile_coords_tile_y(&self) -> u16 {
        self.tile_coords.get_tile_y()
    }

    pub fn get_map_coords_lat(&self) -> f64 {
        self.map_coords.get_lat()
    }
    pub fn get_map_coords_lng(&self) -> f64 {
        self.map_coords.get_lng()
    }
    pub fn get_map_coords_zoom(&self) -> f32 {
        self.map_coords.get_zoom()
    }
}
