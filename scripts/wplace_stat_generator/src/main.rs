use crate::template_image_read::TemplateImageRead;
use std::{fs::OpenOptions, io::Write};

mod art_data;
mod template_image_read;

fn main() {
    let mut out = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("./out.md")
        .expect("Couldn't open file");

    let data = wplace_common::art_data::ArtData::read(wplace_common::ART_FILE);

    out.write_all(
        (data
            .iter()
            .map(|v| art_data::to_markdown_titles_str(v) + "\n")
            .collect::<String>()
            + "\n\n")
            .as_bytes(),
    )
    .expect("Error writing to out file");
    out.write_all(
        data.iter()
            .map(|v| {
                art_data::to_markdown_str(v)
                    + "\n"
                    + TemplateImageRead::image_calc(&format!(
                        "../../templates/wplace/{}",
                        v.get_image_file_name()
                    ))
                    .to_markdown_str()
                    .as_str()
                    + "\n"
            })
            .collect::<String>()
            .as_bytes(),
    )
    .expect("Error writing to out file");
}
