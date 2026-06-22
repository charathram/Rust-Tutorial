// Starting point for Lesson 4: the finished rwc from Lesson 3 (tests).
// You will grow it to count several files and print a grand total —
// meeting shared (&) and mutable (&mut) borrows as the feature demands them.
use std::env;
use std::fs;
use std::error::Error;

fn counts(text: &str) -> (usize, usize, usize) {
    let lines = text.lines().count();
    let words = text.split_whitespace().count();
    let chars = text.chars().count();
    (lines, words, chars)
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = match env::args().nth(1) {
        Some(path) => path,
        None => {
            eprintln!("usage: rwc <file>");
            std::process::exit(2);
        }
    };
    let text = fs::read_to_string(&path).map_err(|e| format!("could not read file: {}", e))?;

    let (lines, words, chars) = counts(&text);

    println!("{lines:>6} {words:>6} {chars:>6} {path}");
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
        assert!(chars < text.len(), "chars ({chars}) should be fewer than bytes ({})", text.len());
    }

    #[test]
    fn counts_empty() {
        assert_eq!(counts(""), (0, 0, 0));
    }
}
