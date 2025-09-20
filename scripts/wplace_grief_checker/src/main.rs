use std::io::{Write, stdin, stdout};

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

        let overwrite = if data.get_incorrect_px_count() > 0 {
            print!(
                "{} has {}px wrong, mark as OK? [Y/?]\n > ",
                v.get_title(),
                data.get_incorrect_px_count()
            );
            stdout().flush().expect("Flush stdout");
            let mut buf = String::new();
            stdin().read_line(&mut buf).expect("Read line");
            buf.contains('Y') || buf.contains('y')
        } else {
            false
        };

        out.write_all((data.to_markdown_str(v, overwrite) + "\n").as_bytes())
            .expect("Error writing to out file");
    }
}
