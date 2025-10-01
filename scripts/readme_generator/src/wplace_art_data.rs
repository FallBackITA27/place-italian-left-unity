use wplace_common::art_data::ArtData;

pub fn to_markdown_whole_list(v: &mut Vec<ArtData>) -> String {
    v.sort_by(|a, b| {
        a.get_alliance()
            .partial_cmp(&b.get_alliance())
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    let mut out = String::new();
    let mut last_alliance = None;
    for item in v {
        if last_alliance != Some(item.get_alliance()) {
            out += item.get_alliance().to_markdown_alliance_list().as_str();
        }

        last_alliance = Some(item.get_alliance());

        out += to_markdown_titles_str(item).as_str();
    }
    out
}

pub fn to_markdown_titles_str(v: &ArtData) -> String {
    format!(
        "    1. [{}](#{})\n",
        v.get_title(),
        v.get_title()
            .to_lowercase()
            .replace(" ", "-")
            .replace("(", "")
            .replace(")", "")
            .replace(",", "")
            .replace("!", "")
    )
}

pub fn to_markdown_str(v: &ArtData) -> String {
    let tile_coords = v.get_tile_coords();
    format!(
        "### {title}\n![{title}]({path})\n\n- Coordinate: [{tl_x} {tl_y} {x} {y}]({map_link})\n- Link Template: [{path}]({path})",
        title = v.get_title(),
        path = String::from("/templates/wplace/") + v.get_image_info().get_file_name(),
        tl_x = tile_coords.get_tile_x(),
        tl_y = tile_coords.get_tile_y(),
        x = tile_coords.get_x(),
        y = tile_coords.get_y(),
        map_link = v.get_map_coords().get_link()
    )
}
