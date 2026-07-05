use rwc::{Counts, Report, Source};
use std::env;
use std::error::Error;
// use std::fs;

// fn counts(text: &str) -> (usize, usize, usize) {
//     let lines = text.lines().count();
//     let words = text.split_whitespace().count();
//     let chars = text.chars().count();
//     (lines, words, chars)
// }

// fn add_into(total: &mut (usize, usize, usize), counts: (usize, usize, usize)) {
//     total.0 += counts.0;
//     total.1 += counts.1;
//     total.2 += counts.2;
// }

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().skip(1).collect();
    let sources = Source::from_args(args)?;

    let mut total = Counts::default();
    for source in &sources {
        let text = source
            .read()
            .map_err(|e| format!("could not read {}: {}", source.name(), e))?;
        let counts = Counts::from_text(&text);
        let label = source.describe();
        let report = Report::new(&label, counts);
        println!("{}", report.line());
        total.add(counts);
    }
    if sources.len() > 1 {
        println!("{}", Report::new("total", total).line());
    }
    Ok(())
}
