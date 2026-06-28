use std::fs;
use std::io::{self, Read};

/// A struct to hold counts of lines, words, and bytes.
///
/// ```
/// use rwc::Counts;
/// let text = "hello world\nthis is a test";
/// let counts = Counts::from_text(text);
/// assert_eq!(counts.lines, 2);
/// assert_eq!(counts.words, 6);
/// assert_eq!(counts.bytes, 26);
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Counts {
    pub lines: usize,
    pub words: usize,
    pub bytes: usize,
}

impl Counts {
    /// Counts the number of lines, words, and bytes in the given text.
    ///
    /// ```
    /// use rwc::Counts;
    /// let text = "hello world\nthis is a test";
    /// let counts = Counts::from_text(text);
    /// assert_eq!(counts.lines, 2);
    /// assert_eq!(counts.words, 6);
    /// assert_eq!(counts.bytes, 26);
    /// ```
    pub fn from_text(text: &str) -> Counts {
        Counts {
            lines: text.lines().count(),
            words: text.split_whitespace().count(),
            bytes: text.len(),
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
    /// ```
    pub fn add(&mut self, other: Counts) {
        self.lines += other.lines;
        self.words += other.words;
        self.bytes += other.bytes;
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
/// An enum to represent the source of text input, either from a file or standard input.
/// ```
/// use rwc::Source;
/// let sources = Source::from_args(vec!["file1.txt".to_string(), "file2.txt".to_string()]);
/// assert_eq!(sources.len(), 2);
/// let sources = Source::from_args(vec![]);
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
    /// let sources = Source::from_args(vec!["file1.txt".to_string(), "file2.txt".to_string()]);
    /// assert_eq!(sources.len(), 2);
    /// let sources = Source::from_args(vec![]);
    /// assert_eq!(sources.len(), 1);
    /// assert_eq!(sources[0], Source::Stdin);
    /// ```
    pub fn from_args(args: Vec<String>) -> Vec<Source> {
        if args.is_empty() {
            vec![Source::Stdin]
        } else {
            args.into_iter().map(Source::File).collect()
        }
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
    }

    #[test]
    fn from_args_empty_defaults_to_stdin() {
        let sources = Source::from_args(vec![]);
        assert_eq!(sources.len(), 1);
        assert_eq!(sources[0], Source::Stdin);
    }

    #[test]
    fn from_args_map_paths_to_files() {
        let sources = Source::from_args(vec!["file1.txt".to_string(), "file2.txt".to_string()]);
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
}
