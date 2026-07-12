use clap::Parser;
use rwc::{Columns, Counts, Report, Source};
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

/// Count lines, words, bytes, characters, and graphemes.
///
/// With no count flags, every column is shown. Pass one or more flags to
/// print only those columns.
#[derive(Parser)]
#[command(version, about, long_about)]
struct Cli {
    /// Show the line count
    #[arg(short, long)]
    lines: bool,

    /// Show the word count
    #[arg(short, long)]
    words: bool,

    /// Show the byte count
    #[arg(short = 'c', long)]
    bytes: bool,

    /// Show the character count
    #[arg(short = 'm', long)]
    chars: bool,

    /// Show the graphemes count
    #[arg(short = 'g', long)]
    graphemes: bool,

    /// Files to count; omit or use '-' for standard input
    paths: Vec<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let cols = Columns::from_flags(cli.lines, cli.words, cli.bytes, cli.chars, cli.graphemes);
    let sources = Source::from_args(cli.paths)?;

    let mut total = Counts::default();
    for source in &sources {
        let text = source
            .read()
            .map_err(|e| format!("could not read {}: {}", source.name(), e))?;
        let counts = Counts::from_text(&text);
        let label = source.describe();
        let report = Report::new(&label, counts);
        println!("{}", report.line(&cols));
        total.add(counts);
    }
    if sources.len() > 1 {
        println!("{}", Report::new("total", total).line(&cols));
    }
    Ok(())
}
