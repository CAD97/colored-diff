extern crate ansi_term;
extern crate difference;
extern crate itertools;

use ansi_term::{ANSIGenericString, Colour};
use difference::{Changeset, Difference};
use itertools::Itertools;
use std::{fmt, sync::Once};

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

#[inline(always)]
fn enable_ansi() {
    if cfg!(windows) {
        static ONCE: Once = Once::new();
        ONCE.call_once(|| {ansi_term::enable_ansi_support().ok();});
    }
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
    let changeset = Changeset::new(expected, actual, "\n");
    fmt_changeset(f, &changeset)
}

fn fmt_changeset(f: &mut fmt::Formatter, changeset: &Changeset) -> fmt::Result {
    enable_ansi();

    writeln!(f, "{} {} / {} {}",
        red(LEFT), red("left"),
        green(RIGHT), green("right"),
    )?;

    let diffs = &changeset.diffs;
    for (i, diff) in diffs.iter().enumerate() {
        match diff {
            Difference::Same(text) => {
                format_same(f, text)?;
            }
            Difference::Add(added) => {
                if let Some(Difference::Rem(removed)) = diffs.get(i - 1) {
                    format_add_rem(f, added, removed)?;
                } else {
                    format_add(f, added)?;
                }
            }
            Difference::Rem(removed) => {
                if let Some(Difference::Add(_)) = diffs.get(i + 1) {
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
    let Changeset { diffs, .. } = Changeset::new(removed, added, "");

    // LEFT (removed)
    write!(f, "{}", red(LEFT))?;
    for diff in &diffs {
        match diff {
            Difference::Same(text) => {
                for blob in text.split('\n').intersperse(NL_LEFT) {
                    write!(f, "{}", red(blob))?;
                }
            }
            Difference::Rem(text) => {
                for blob in text.split('\n').intersperse(NL_LEFT) {
                    write!(f, "{}", on_red(blob))?;
                }
            }
            Difference::Add(_) => continue,
        }
    }
    writeln!(f)?;

    // RIGHT (added)
    write!(f, "{}", green(RIGHT))?;
    for diff in &diffs {
        match diff {
            Difference::Same(text) => {
                for blob in text.split('\n').intersperse(NL_RIGHT) {
                    write!(f, "{}", green(blob))?;
                }
            }
            Difference::Add(text) => {
                for blob in text.split('\n').intersperse(NL_RIGHT) {
                    write!(f, "{}", on_green(blob))?;
                }
            }
            Difference::Rem(_) => continue,
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
