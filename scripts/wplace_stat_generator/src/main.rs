use std::io::Write;

use crate::template_image_read::TemplateImageRead;

mod art_data;
mod color;
mod template_image_read;

fn main() {
    let art_data_file = include_str!("../art");

    let mut out = std::fs::File::create_new("./out.md").expect("out file exists already");

    let data = art_data::ArtDataReader::read(art_data_file);

    out.write_all(
        (data
            .iter()
            .map(|v| v.to_markdown_titles_str() + "\n")
            .collect::<String>()
            + "\n\n")
            .as_bytes(),
    )
    .expect("Error writing to out file");
    out.write_all(
        data.iter()
            .map(|v| {
                v.to_markdown_str()
                    + "\n"
                    + TemplateImageRead::image_calc(&format!("../../templates/wplace/{}", v.get_path()))
                        .to_markdown_str()
                        .as_str()
                    + "\n"
            })
            .collect::<String>()
            .as_bytes(),
    )
    .expect("Error writing to out file");
}
