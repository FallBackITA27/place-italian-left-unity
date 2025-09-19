use wplace_common::art_data::ArtData;

pub fn to_markdown_titles_str(v: &ArtData) -> String {
    format!(
        "    1. [{}](#{})",
        v.get_title(),
        v.get_title()
            .to_lowercase()
            .replace(" ", "-")
            .replace("(", "")
            .replace(")", "")
            .replace(",", "")
    )
}

pub fn to_markdown_str(v: &ArtData) -> String {
    format!(
        "### {title}\n![{title}]({path})\n\n- Coordinate: [{tl_x} {tl_y} {x} {y}](https://wplace.live/?lat={lat}&lng={lng}&zoom={zoom})\n- Link Template: [{path}]({path})",
        title = v.get_title(),
        path = v.get_image_file_name(),
        tl_x = v.get_tile_coords_tile_x(),
        tl_y = v.get_tile_coords_tile_y(),
        x = v.get_tile_coords_x(),
        y = v.get_tile_coords_y(),
        lat = v.get_map_coords_lat(),
        lng = v.get_map_coords_lng(),
        zoom = v.get_map_coords_zoom(),
    )
}
