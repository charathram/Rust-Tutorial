// Starting point for Lesson 5: the finished rwc from Lesson 4 (borrowing).
// It counts several files and prints a grand total. In this lesson you will
// deliberately break it to provoke real borrow-checker errors (E03xx),
// learn to read each one, and apply the standard fix.
use std::env;
use std::error::Error;
use std::fs;

fn counts(text: &str) -> (usize, usize, usize) {
    let lines = text.lines().count();
    let words = text.split_whitespace().count();
    let chars = text.chars().count();
    (lines, words, chars)
}

fn add_into(total: &mut (usize, usize, usize), counts: (usize, usize, usize)) {
    total.0 += counts.0;
    total.1 += counts.1;
    total.2 += counts.2;
}

fn main() -> Result<(), Box<dyn Error>> {
    let paths: Vec<String> = env::args().skip(1).collect();
    if paths.is_empty() {
        eprintln!("Usage: rwc <file> [<file> ...]");
        std::process::exit(2);
    }

    let mut total = (0, 0, 0);
    for path in &paths {
        let text =
            fs::read_to_string(path).map_err(|e| format!("could not read file {}: {}", path, e))?;
        let (lines, words, chars) = counts(&text);
        println!("{lines:>6} {words:>6} {chars:>6} {path}");
        add_into(&mut total, (lines, words, chars));
    }
    if paths.len() > 1 {
        println!("{:>6} {:>6} {:>6} total", total.0, total.1, total.2);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_ascii() {
        let text = "hello world\nthis is a test";
        let (lines, words, chars) = counts(text);
        assert_eq!(lines, 2);
        assert_eq!(words, 6);
        assert_eq!(chars, 26);
    }

    #[test]
    fn counts_unicode() {
        let text = "வணக்கம்\n";
        let (lines, words, chars) = counts(text);
        assert_eq!(lines, 1);
        assert_eq!(words, 1);
        assert!(
            chars < text.len(),
            "chars ({chars}) should be fewer than bytes ({})",
            text.len()
        );
    }

    #[test]
    fn counts_empty() {
        assert_eq!(counts(""), (0, 0, 0));
    }

    #[test]
    fn add_into_accumulates() {
        let mut total = (0, 0, 0);
        add_into(&mut total, (1, 2, 3));
        add_into(&mut total, (4, 5, 6));
        assert_eq!(total, (5, 7, 9));
    }
}
