use crate::grief_checker::GriefChecker;
use std::{
    fs::OpenOptions,
    io::{Write, stdin, stdout},
};

mod grief_checker;

#[tokio::main]
async fn main() {
    let mut out = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("./out.md")
        .expect("Couldn't open file");
    let data = wplace_common::art_data::ArtData::read(wplace_common::ART_FILE);

    out.write_all(b"# Status Art\n")
        .expect("Error writing to out file");

    for v in data.iter() {
        let data = GriefChecker::check(v).await;

        let overwrite = if data.get_incorrect_px_count() > 0 {
            print!(
                "{} has {}px wrong, mark as OK? [Y/N]\n > ",
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

        if !overwrite && data.get_incorrect_px_count() > 0 {
            print!("Write down wrong PX coordinates? [Y/N]\n > ",);
            stdout().flush().expect("Flush stdout");

            let mut buf = String::new();
            stdin().read_line(&mut buf).expect("Read line");

            if buf.contains('Y') || buf.contains('y') {
                out.write_all(data.print_wrong_px_coords().as_bytes())
                    .expect("Error writing to out file");
            }
        }
    }
}
