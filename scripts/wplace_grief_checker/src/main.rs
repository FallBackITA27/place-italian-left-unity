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
    let mut data = wplace_common::art_data::ArtData::new();
    data.sort_by(|a, b| {
        a.get_alliance()
            .partial_cmp(&b.get_alliance())
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    out.write_all(b"# Grief Check!\n")
        .expect("Error writing to out file");

    let mut summary_time = 0.0;
    let mut total_px = 0;
    let mut total_missing_px = 0;

    for v in data.iter() {
        let data = GriefChecker::new().check(v).await;

        total_px += data.get_total_px();

        if data.get_incorrect_px_count() == 0 {
            continue;
        }

        print!(
            "{} ({}) has {}px wrong, mark as OK? [Y/N]\n > ",
            v.get_title(),
            v.get_map_coords().get_link(),
            data.get_incorrect_px_count()
        );
        stdout().flush().expect("Flush stdout");

        let mut buf = String::new();

        stdin().read_line(&mut buf).expect("Read line");
        if buf.contains('Y') || buf.contains('y') {
            continue;
        }

        out.write_all((data.to_markdown_str(v) + "\n").as_bytes())
            .expect("Error writing to out file");

        summary_time += data.get_missing_time_hrs();
        total_missing_px += data.get_incorrect_px_count();

        print!("Write down wrong PX coordinates? [Y/N]\n > ",);
        stdout().flush().expect("Flush stdout");

        stdin().read_line(&mut buf).expect("Read line");
        if buf.contains('Y') || buf.contains('y') {
            out.write_all(data.print_wrong_px_coords().as_bytes())
                .expect("Error writing to out file");
        }
    }

    out.write_all(
        format!(
            "## Sommario: {summary_time:.1}h, manca il {:.2}%\n",
            (total_missing_px as f64) / (total_px as f64) * 100.0
        )
        .as_bytes(),
    )
    .expect("Error writing to file");
}
