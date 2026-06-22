use std::env;
use std::fs;

fn main() {
    let path = env::args().nth(1).expect("usage: rwc <file>");
    let text = fs::read_to_string(&path).expect("Could not read file");

    let lines = text.lines().count();
    let words = text.split_whitespace().count();
    let chars = text.chars().count();
    let longest = text.lines().map(|line| line.chars().count()).max();

    println!("{lines:>6} {words:>6} {chars:>6} {path}");
    println!("Longest line: {longest:?} characters");
}
