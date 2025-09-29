use wplace_common::art_data::Alliances;

use crate::wplace_template_image_read::TemplateImageRead;
use std::{fs::OpenOptions, io::Write};

mod wplace_art_data;
mod wplace_template_image_read;

fn main() {
    let mut out = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("./out.md")
        .expect("Couldn't open file");

    let data = wplace_common::art_data::ArtData::new();

    let mut ilu_wplace_art_data = Vec::new();
    let mut brindisiplace_wplace_art_data = Vec::new();

    for art_data in data {
        match art_data.get_alliance() {
            Alliances::None => ilu_wplace_art_data.push(art_data),
            Alliances::BrindisiPlace => brindisiplace_wplace_art_data.push(art_data),
        }
    }

    out.write_all(
        format!(
            include_str!("../template_README.md"),
            wplace_project_list_points = ilu_wplace_art_data
                .iter()
                .map(|v| wplace_art_data::to_markdown_titles_str(v) + "\n")
                .collect::<String>()
                .trim_end(),
            wplace_allies_list_points = Alliances::markdown_list().trim_end(),
            wplace_project_list = ilu_wplace_art_data
                .iter()
                .map(|v| {
                    wplace_art_data::to_markdown_str(v)
                        + "\n"
                        + TemplateImageRead::image_calc(v).to_markdown_str().as_str()
                        + "\n"
                })
                .collect::<String>()
                .trim_end(),
            wplace_brindisiplace_project_list = brindisiplace_wplace_art_data
                .iter()
                .map(|v| {
                    wplace_art_data::to_markdown_str(v)
                        + "\n"
                        + TemplateImageRead::image_calc(v).to_markdown_str().as_str()
                        + "\n"
                })
                .collect::<String>()
                .trim_end(),
            wplace_griefer_list = wplace_common::griefer_data::GrieferData::new()
                .iter()
                .map(|v| v.to_markdown_list())
                .collect::<String>()
                .trim_end(),
        )
        .as_bytes(),
    )
    .expect("Error writing to out file");
}
