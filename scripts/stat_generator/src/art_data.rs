pub struct TileCoords {
    tile_x: u16,
    tile_y: u16,
    x: u16,
    y: u16,
}

pub struct MapCoords {
    lat: f64,
    lng: f64,
    zoom: f32,
}

pub struct ArtDataReader {
    title: String,
    img_path: String,
    tile_coords: TileCoords,
    map_coords: MapCoords,
}

impl ArtDataReader {
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
                img_path: img_path.to_string(),
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

    pub fn get_path<'a>(&'a self) -> &'a str {
        &self.img_path
    }

    pub fn to_markdown_titles_str(&self) -> String {
        format!(
            "    1. [{}](#{})",
            self.title,
            self.title
                .to_lowercase()
                .replace(" ", "-")
                .replace("(", "")
                .replace(")", "")
                .replace(",", "")
        )
    }

    pub fn to_markdown_str(&self) -> String {
        format!(
            "### {title}\n![{title}]({path})\n\n- Coordinate: [{tl_x} {tl_y} {x} {y}](https://wplace.live/?lat={lat}&lng={lng}&zoom={zoom})\n- Link Template: [{path}]({path})",
            title = self.title,
            path = self.img_path,
            tl_x = self.tile_coords.tile_x,
            tl_y = self.tile_coords.tile_y,
            x = self.tile_coords.x,
            y = self.tile_coords.y,
            lat = self.map_coords.lat,
            lng = self.map_coords.lng,
            zoom = self.map_coords.zoom,
        )
    }
}
