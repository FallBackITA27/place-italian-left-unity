pub struct TileCoords {
    tile_x: u16,
    tile_y: u16,
    x: u16,
    y: u16,
}

impl TileCoords {
    pub fn new(tile_x: u16, tile_y: u16, x: u16, y: u16) -> Self {
        TileCoords {
            tile_x,
            tile_y,
            x,
            y,
        }
    }

    pub fn parse_tile_coords_string(v: &str) -> Self {
        let mut tile_coords = v
            .trim()
            .strip_prefix('(')
            .expect("Couldn't strip prefix tile coords")
            .strip_suffix(')')
            .expect("Couldn't strip suffix tile coords")
            .split(',')
            .map(|x| {
                x.split(':')
                    .next_back()
                    .expect("Deformed tile coords data")
                    .trim()
                    .parse::<u16>()
                    .expect("No number parsing tile coords")
            });
        Self {
            tile_x: tile_coords.next().expect("No Tile X coords found"),
            tile_y: tile_coords.next().expect("No Tile Y coords found"),
            x: tile_coords.next().expect("No X coords found"),
            y: tile_coords.next().expect("No Y coords found"),
        }
    }

    pub fn get_x(&self) -> u16 {
        self.x
    }
    pub fn get_y(&self) -> u16 {
        self.y
    }
    pub fn get_tile_x(&self) -> u16 {
        self.tile_x
    }
    pub fn get_tile_y(&self) -> u16 {
        self.tile_y
    }
}
