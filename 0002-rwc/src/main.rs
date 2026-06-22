// Starting point for Lesson 2: the finished rwc from Lesson 1.
// You will harden this — replacing the .expect() panics with real error handling.
use std::env;
use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // let path = env::args().nth(1).ok_or("usage: rwc <file>")?;
    let path = match env::args().nth(1) {
        Some(path) => path,
        None => {
            eprintln!("usage: rwc <file>");
            std::process::exit(2);
        }
    };
    let text = fs::read_to_string(&path).map_err(|e| format!("could not read file: {}", e))?;


    let lines = text.lines().count();
    let words = text.split_whitespace().count();
    let chars = text.chars().count();

    println!("{lines:>6} {words:>6} {chars:>6} {path}");
    Ok(())
}
