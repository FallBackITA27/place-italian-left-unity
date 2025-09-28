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

    let data = wplace_common::art_data::ArtData::new();

    out.write_all(
        format!(
            include_str!("../template_README.md"),
            wplace_project_list_points = data
                .iter()
                .map(|v| art_data::to_markdown_titles_str(v) + "\n")
                .collect::<String>()
                .trim_end(),
            wplace_project_list = data
                .iter()
                .map(|v| {
                    art_data::to_markdown_str(v)
                        + "\n"
                        + TemplateImageRead::image_calc(v).to_markdown_str().as_str()
                        + "\n"
                })
                .collect::<String>()
                .trim_end()
        )
        .as_bytes(),
    )
    .expect("Error writing to out file");
}
