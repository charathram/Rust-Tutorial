use std::fs;
use std::io::{self, Read};
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Columns {
    pub lines: bool,
    pub words: bool,
    pub bytes: bool,
    pub chars: bool,
    pub graphemes: bool,
}

impl Columns {
    /// Every column is selected.
    pub fn all() -> Columns {
        Columns {
            lines: true,
            words: true,
            bytes: true,
            chars: true,
            graphemes: true,
        }
    }

    /// Builds a `Columns` instance from command-line arguments, where each argument is a single character representing a column to include. If no arguments are provided, all columns are selected by default.
    pub fn from_flags(
        lines: bool,
        words: bool,
        bytes: bool,
        chars: bool,
        graphemes: bool,
    ) -> Columns {
        if !(lines || words || bytes || chars || graphemes) {
            return Columns::all();
        } else {
            Columns {
                lines,
                words,
                bytes,
                chars,
                graphemes,
            }
        }
    }
}

pub struct Report<'a> {
    label: &'a str,
    counts: Counts,
}

impl<'a> Report<'a> {
    pub fn new(label: &'a str, counts: Counts) -> Report<'a> {
        Report { label, counts }
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn counts(&self) -> Counts {
        self.counts
    }

    /// Formats the report as one output row: the four counts, then the label.
    ///
    /// ```
    /// use rwc::{Counts, Report, Columns};
    /// let report = Report::new("-", Counts::from_text("a b c\n"));
    /// let cols = Columns::all();
    /// assert_eq!(report.line(&cols), "     1      3      6      6      6 -");
    /// ```
    pub fn line(&self, cols: &Columns) -> String {
        let mut parts: Vec<String> = Vec::new();
        if cols.lines {
            parts.push(format!("{:>6}", self.counts.lines));
        }
        if cols.words {
            parts.push(format!("{:>6}", self.counts.words));
        }
        if cols.bytes {
            parts.push(format!("{:>6}", self.counts.bytes));
        }
        if cols.chars {
            parts.push(format!("{:>6}", self.counts.chars));
        }
        if cols.graphemes {
            parts.push(format!("{:>6}", self.counts.graphemes));
        }
        parts.push(self.label.to_string());
        parts.join(" ")
    }
}

/// Returns the first `n` characters of `s` as a borrowed `&str`.
/// Counting is by character, so the slice always ends on a boundary and
/// never panics. Fewer than `n` characters returns the whole string.
///
/// ```
/// use rwc::first_chars;
/// assert_eq!(first_chars("hello", 2), "he");
/// assert_eq!(first_chars("café", 4), "café"); // 4 chars, 5 bytes — no panic
/// assert_eq!(first_chars("hi", 5), "hi");     // fewer than n chars → all of it
/// ```
pub fn first_chars<'a>(s: &'a str, n: usize) -> &'a str {
    match s.char_indices().nth(n) {
        Some((idx, _)) => &s[..idx],
        None => s,
    }
}

/// A struct to hold counts of lines, words, bytes, characters, and graphemes.
/// `chars` counts Unicode scalar values (`char`); `graphemes` counts
/// user-perceived characters (grapheme clusters), so an accented letter
/// written as a base plus a combining mark counts as two `chars` but one
/// grapheme. `graphemes <= chars <= bytes` always holds.
/// ```
/// use rwc::Counts;
/// let text = "hello world\nthis is a test";
/// let counts = Counts::from_text(text);
/// assert_eq!(counts.lines, 2);
/// assert_eq!(counts.words, 6);
/// assert_eq!(counts.bytes, 26);
/// assert_eq!(counts.chars, 26);
/// assert_eq!(counts.graphemes, 26);
/// // "e" + U+0301 (combining acute) renders as "é": 3 bytes, 2 chars, 1 grapheme.
/// let combined = Counts::from_text("e\u{301}");
/// assert_eq!(combined.bytes, 3);
/// assert_eq!(combined.chars, 2);
/// assert_eq!(combined.graphemes, 1);
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Counts {
    pub lines: usize,
    pub words: usize,
    pub bytes: usize,
    pub chars: usize,
    pub graphemes: usize,
}

impl Counts {
    /// Counts the number of lines, words, characters, graphemes, and bytes in
    /// the given text.
    /// `chars` counts Unicode scalar values (`char`); `graphemes` counts
    /// user-perceived characters (grapheme clusters), so an accented letter
    /// written as a base plus a combining mark counts as two `chars` but one
    /// grapheme. `graphemes <= chars <= bytes` always holds.
    ///
    /// ```
    /// use rwc::Counts;
    /// let text = "hello world\nthis is a test";
    /// let counts = Counts::from_text(text);
    /// assert_eq!(counts.lines, 2);
    /// assert_eq!(counts.words, 6);
    /// assert_eq!(counts.bytes, 26);
    /// assert_eq!(counts.chars, 26);
    /// assert_eq!(counts.graphemes, 26);
    /// // "e" + U+0301 (combining acute) renders as "é": 3 bytes, 2 chars, 1 grapheme.
    /// let combined = Counts::from_text("e\u{301}");
    /// assert_eq!(combined.bytes, 3);
    /// assert_eq!(combined.chars, 2);
    /// assert_eq!(combined.graphemes, 1);
    /// ```
    pub fn from_text(text: &str) -> Counts {
        Counts {
            lines: text.lines().count(),
            words: text.split_whitespace().count(),
            bytes: text.len(),
            chars: text.chars().count(),
            graphemes: text.graphemes(true).count(),
        }
    }

    /// Adds the counts from another `Counts` instance into this one.
    ///
    /// ```
    /// use rwc::Counts;
    /// let mut total = Counts::default();
    /// total.add(Counts::from_text("hello world\nthis is a test"));
    /// assert_eq!(total.lines, 2);
    /// assert_eq!(total.words, 6);
    /// assert_eq!(total.bytes, 26);
    /// assert_eq!(total.chars, 26);
    /// ```
    pub fn add(&mut self, other: Counts) {
        self.lines += other.lines;
        self.words += other.words;
        self.bytes += other.bytes;
        self.chars += other.chars;
        self.graphemes += other.graphemes;
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
/// An enum to represent the source of text input, either from a file or standard input.
/// ```
/// use rwc::Source;
/// let sources = Source::from_args(vec!["file1.txt".to_string(), "file2.txt".to_string()]).unwrap();
/// assert_eq!(sources.len(), 2);
/// let sources = Source::from_args(vec![]).unwrap();
/// assert_eq!(sources.len(), 1);
/// assert_eq!(sources[0], Source::Stdin);
/// ```
pub enum Source {
    File(String),
    #[default]
    Stdin,
}

impl Source {
    /// Creates a vector of `Source` instances from command-line arguments.
    ///
    /// If no arguments are provided, it defaults to standard input.
    ///
    /// ```
    /// use rwc::Source;
    /// let sources = Source::from_args(vec!["file1.txt".to_string(), "file2.txt".to_string()]).unwrap();
    /// assert_eq!(sources.len(), 2);
    /// let sources = Source::from_args(vec![]).unwrap();
    /// assert_eq!(sources.len(), 1);
    /// assert_eq!(sources[0], Source::Stdin);
    /// // A lone "-" means standard input.
    /// let sources = Source::from_args(vec!["-".to_string()]).unwrap();
    /// assert_eq!(sources.len(), 1);
    /// assert_eq!(sources[0], Source::Stdin);
    /// // An argument starting with "-" (other than "-" itself) is rejected.
    /// let result = Source::from_args(vec!["-invalid".to_string()]);
    /// assert!(result.is_err());
    /// ```
    pub fn from_args(args: Vec<String>) -> Result<Vec<Source>, Box<dyn std::error::Error>> {
        if args.is_empty() {
            return Ok(vec![Source::Stdin]);
        }
        args.into_iter()
            .map(|arg| match arg.as_str() {
                "-" => Ok(Source::Stdin),
                flag if flag.starts_with('-') => Err(format!("invalid argument: {flag}").into()),
                _ => Ok(Source::File(arg)), // String is an open set, so a catchall is required here.
            })
            .collect()
    }

    /// Returns the name of the source, which is either the file path or "-" for standard input.
    /// ```
    /// use rwc::Source;
    /// let source = Source::File("file1.txt".to_string());
    /// assert_eq!(source.name(), "file1.txt");
    /// let source = Source::Stdin;
    /// assert_eq!(source.name(), "-");
    /// ```
    pub fn name(&self) -> &str {
        match self {
            Source::File(path) => path,
            Source::Stdin => "-",
        }
    }

    /// Reads the content from the source, returning it as a `String`.
    ///
    /// If the source is a file, it reads the file's content. If it's standard input, it reads from stdin until EOF.
    ///
    /// ```no_run
    /// use rwc::Source;
    /// let text = Source::Stdin.read()?;
    /// println!("{} bytes", text.len());
    /// # Ok::<(), std::io::Error>(())
    /// ```
    pub fn read(&self) -> io::Result<String> {
        match self {
            Source::File(path) => fs::read_to_string(path),
            Source::Stdin => {
                let mut buffer = String::new();
                io::stdin().read_to_string(&mut buffer)?;
                Ok(buffer)
            }
        }
    }

    /// Returns a descriptive string for the source, indicating if it's a markdown file or not.
    ///
    /// ```
    /// use rwc::Source;
    /// assert_eq!(Source::File("file1.md".to_string()).describe(), "file1.md (markdown)");
    /// assert_eq!(Source::File("file2.txt".to_string()).describe(), "file2.txt");
    /// assert_eq!(Source::Stdin.describe(), "-");
    /// ```
    pub fn describe(&self) -> String {
        match self {
            Source::File(path) if path.ends_with(".md") => format!("{path} (markdown)"),
            Source::File(path) => path.clone(),
            Source::Stdin => "-".to_string(),
        }
    }

    /// Returns the file path if the source is a file, or "-" if it's standard input.
    ///
    /// ```
    /// use rwc::Source;
    /// assert_eq!(Source::File("file1.txt".to_string()).path_or_dash(), "file1.txt");
    /// assert_eq!(Source::Stdin.path_or_dash(), "-");
    /// ```
    pub fn path_or_dash(&self) -> String {
        let Source::File(path) = self else {
            return "-".to_string();
        };
        path.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_ascii() {
        let c = Counts::from_text("hello world\nthis is a test");
        assert_eq!(c.lines, 2);
        assert_eq!(c.words, 6);
        assert_eq!(c.bytes, 26);
    }

    #[test]
    fn counts_unicode() {
        let text = "வணக்கம்\n";
        let c = Counts::from_text(text);
        assert_eq!(c.lines, 1);
        assert_eq!(c.words, 1);
        // "வணக்கம்" is 7 Tamil scalar values at 3 UTF-8 bytes each, plus the
        // newline = 22 bytes, but only 8 characters — so bytes exceed chars.
        assert_eq!(c.bytes, text.len());
        assert_eq!(c.chars, text.chars().count());
        assert!(
            c.bytes > text.chars().count(),
            "bytes ({}) should exceed chars ({})",
            c.bytes,
            text.chars().count()
        );
    }

    #[test]
    fn counts_empty() {
        let c = Counts::from_text("");
        assert_eq!(c.lines, 0);
        assert_eq!(c.words, 0);
        assert_eq!(c.bytes, 0);
        assert_eq!(c.chars, 0);
    }

    #[test]
    fn from_args_empty_defaults_to_stdin() {
        let sources = Source::from_args(vec![]).unwrap();
        assert_eq!(sources.len(), 1);
        assert_eq!(sources[0], Source::Stdin);
    }

    #[test]
    fn from_args_map_paths_to_files() {
        let sources =
            Source::from_args(vec!["file1.txt".to_string(), "file2.txt".to_string()]).unwrap();
        assert_eq!(sources.len(), 2);
        assert_eq!(sources[0], Source::File("file1.txt".to_string()));
        assert_eq!(sources[1], Source::File("file2.txt".to_string()));
    }

    #[test]
    fn add_accumulates_counts() {
        let mut total = Counts::default();
        total.add(Counts::from_text("hello world\nthis is a test"));
        assert_eq!(total.lines, 2);
        assert_eq!(total.words, 6);
        assert_eq!(total.bytes, 26);
        total.add(Counts::from_text("another line\nand another"));
        assert_eq!(total.lines, 4);
        assert_eq!(total.words, 10);
        assert_eq!(total.bytes, 50);
    }

    #[test]
    fn from_args_dash_is_stdin() {
        let sources = Source::from_args(vec!["-".to_string()]).unwrap();
        assert_eq!(sources.len(), 1);
        assert_eq!(sources[0], Source::Stdin);
    }

    #[test]
    fn from_args_rejects_unknown_flag() {
        let result = Source::from_args(vec!["-x".to_string()]);
        assert!(result.is_err());
    }

    #[test]
    fn add_accumulates_counts_with_chars() {
        let mut total = Counts::default();
        total.add(Counts::from_text("hello world\nthis is a test"));
        assert_eq!(total.lines, 2);
        assert_eq!(total.words, 6);
        assert_eq!(total.bytes, 26);
        assert_eq!(total.chars, 26);
        total.add(Counts::from_text("another line\nand another"));
        assert_eq!(total.lines, 4);
        assert_eq!(total.words, 10);
        assert_eq!(total.bytes, 50);
        assert_eq!(total.chars, 50);
    }

    #[test]
    fn first_chars_respects_boundaries() {
        let s = "café";
        assert_eq!(first_chars(s, 3), "caf");
        assert_eq!(first_chars(s, 4), "café");
        assert_eq!(first_chars(s, 99), "café");
    }

    #[test]
    fn report_borrows_label_and_holds_counts() {
        let label = "demo.txt".to_string();
        let counts = Counts::from_text("a b c\n");
        let report = Report::new(&label, counts);
        assert_eq!(report.label, "demo.txt");
        assert_eq!(report.counts.lines, 1);
        assert_eq!(report.counts.words, 3);
        assert_eq!(report.counts.bytes, 6);
        assert_eq!(report.counts.chars, 6);
        assert_eq!(report.counts.graphemes, 6);
        assert_eq!(
            report.line(&Columns::all()),
            "     1      3      6      6      6 demo.txt"
        );
    }

    #[test]
    fn counts_graphemes_below_chars_for_combining() {
        let c = Counts::from_text("cafe\u{0301}");
        assert_eq!(c.bytes, 6);
        assert_eq!(c.chars, 5);
        assert_eq!(c.graphemes, 4);
    }

    #[test]
    fn line_prints_only_selected_columns() {
        let report = Report::new("f", Counts::from_text("a b c\n"));
        let cols = Columns::from_flags(false, true, true, false, false);

        assert_eq!(report.line(&cols), "     3      6 f");
    }
}
