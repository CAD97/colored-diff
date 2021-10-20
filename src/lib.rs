extern crate ansi_term;
extern crate dissimilar;
extern crate itertools;
#[cfg(test)]
extern crate regex;

use ansi_term::{ANSIGenericString, Colour};
use dissimilar::Chunk;
use itertools::Itertools;
use std::fmt;

fn red(s: &str) -> ANSIGenericString<str> {
    Colour::Red.paint(s)
}
fn on_red(s: &str) -> ANSIGenericString<str> {
    Colour::White.on(Colour::Red).bold().paint(s)
}
fn green(s: &str) -> ANSIGenericString<str> {
    Colour::Green.paint(s)
}
fn on_green(s: &str) -> ANSIGenericString<str> {
    Colour::White.on(Colour::Green).bold().paint(s)
}

static LEFT: &str = "<";
static NL_LEFT: &str = "\n<";
static RIGHT: &str = ">";
static NL_RIGHT: &str = "\n>";

#[cfg(windows)]
#[inline(always)]
fn enable_ansi() {
    use std::sync::Once;

    static ONCE: Once = Once::new();
    ONCE.call_once(|| {ansi_term::enable_ansi_support().ok();});
}

#[cfg(not(windows))]
#[inline(always)]
fn enable_ansi() {
}

#[derive(Copy, Clone, Debug)]
pub struct PrettyDifference<'a> {
    pub expected: &'a str,
    pub actual: &'a str,
}

impl<'a> fmt::Display for PrettyDifference<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        diff(f, self.expected, self.actual)
    }
}

/// Format the difference between strings using GitHub-like formatting with ANSI coloring.
pub fn diff(f: &mut fmt::Formatter, expected: &str, actual: &str) -> fmt::Result {
    let changeset = dissimilar::diff(expected, actual);
    fmt_changeset(f, &changeset)
}

fn fmt_changeset(f: &mut fmt::Formatter, changeset: &Vec<Chunk>) -> fmt::Result {
    enable_ansi();

    writeln!(f, "{} {} / {} {}",
        red(LEFT), red("left"),
        green(RIGHT), green("right"),
    )?;

    for (i, diff) in changeset.iter().enumerate() {
        match diff {
            Chunk::Equal(text) => {
                format_same(f, text)?;
            }
            Chunk::Insert(added) => {
                if let Some(Chunk::Delete(removed)) = i.checked_sub(1).map(|i| &changeset[i]) {
                    format_add_rem(f, added, removed)?;
                } else {
                    format_add(f, added)?;
                }
            }
            Chunk::Delete(removed) => {
                if let Some(Chunk::Insert(_)) = changeset.get(i + 1) {
                    continue;
                } else {
                    format_rem(f, removed)?;
                }
            }
        }
    }

    Ok(())
}

fn format_add_rem(f: &mut fmt::Formatter, added: &str, removed: &str) -> fmt::Result {
    let diffs = dissimilar::diff(removed, added);

    // LEFT (removed)
    write!(f, "{}", red(LEFT))?;
    for diff in &diffs {
        match diff {
            Chunk::Equal(text) => {
                for blob in Itertools::intersperse(text.split('\n'), NL_LEFT) {
                    write!(f, "{}", red(blob))?;
                }
            }
            Chunk::Delete(text) => {
                for blob in Itertools::intersperse(text.split('\n'), NL_LEFT) {
                    write!(f, "{}", on_red(blob))?;
                }
            }
            Chunk::Insert(_) => continue,
        }
    }
    writeln!(f)?;

    // RIGHT (added)
    write!(f, "{}", green(RIGHT))?;
    for diff in &diffs {
        match diff {
            Chunk::Equal(text) => {
                for blob in Itertools::intersperse(text.split('\n'), NL_RIGHT) {
                    write!(f, "{}", green(blob))?;
                }
            }
            Chunk::Insert(text) => {
                for blob in Itertools::intersperse(text.split('\n'), NL_RIGHT) {
                    write!(f, "{}", on_green(blob))?;
                }
            }
            Chunk::Delete(_) => continue,
        }
    }
    writeln!(f)?;

    Ok(())
}

fn format_same(f: &mut fmt::Formatter, text: &str) -> fmt::Result {
    for line in text.split('\n') {
        writeln!(f, " {}", line)?;
    }
    Ok(())
}

fn format_add(f: &mut fmt::Formatter, text: &str) -> fmt::Result {
    for line in text.split('\n') {
        writeln!(f, "{}{}", green(RIGHT), green(line))?;
    }
    Ok(())
}

fn format_rem(f: &mut fmt::Formatter, text: &str) -> fmt::Result {
    for line in text.split('\n') {
        writeln!(f, "{}{}", red(LEFT), red(line))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    use super::*;

    #[test]
    fn single_add() {
        PrettyDifference {
            expected: "",
            actual: "foo",
        }
        .to_string();
    }

    #[test]
    fn color_free_diff() {
        let diff: String = PrettyDifference {
            expected: "a\nb\nc",
            actual: "\nb\ncc",
        }
        .to_string();

        let re = Regex::new(r"\u{1b}\[[0-9;]*m").unwrap();
        assert_eq!(
            re.replace_all(&diff, ""),
            "< left / > right\n<a\n \n b\n c\n>c\n"
        );
    }

    #[test]
    fn color_diff() {
        let diff: String = PrettyDifference {
            expected: "a\nb\nc",
            actual: "\nb\ncc",
        }
        .to_string();

        assert_eq!(diff, "\u{1b}[31m<\u{1b}[0m \u{1b}[31mleft\u{1b}[0m / \u{1b}[32m>\u{1b}[0m \u{1b}[32mright\u{1b}[0m\n\u{1b}[31m<\u{1b}[0m\u{1b}[31ma\u{1b}[0m\n \n b\n c\n\u{1b}[32m>\u{1b}[0m\u{1b}[32mc\u{1b}[0m\n");
    }
}
