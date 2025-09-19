use std::io::Write;

use crate::grief_checker::GriefChecker;

mod grief_checker;

#[tokio::main]
async fn main() {
    let mut out = std::fs::File::create_new("./out.md").expect("out file exists already");
    let data = wplace_common::art_data::ArtData::read(wplace_common::ART_FILE);

    out.write_all(b"# Status Art\n")
        .expect("Error writing to out file");

    for v in data.iter() {
        let data = GriefChecker::check(v).await;
        out.write_all((data.to_markdown_str(v) + "\n").as_bytes())
            .expect("Error writing to out file");
    }
}
