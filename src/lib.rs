//! Functions to find the difference between to texts (strings).
//! Usage
//! ----------
//!
//! Add the following to your Cargo.toml:
//!
//! ```toml
//! [dependencies.text_diff]
//!
//! git = "https://github.com/johannhof/text-diff.rs.git"
//! ```
//!
//! Now you can use the crate in your code
//!
//! ```ignore
//! extern crate text_diff;
//! ```

#![crate_name = "text_diff"]
#![doc(html_root_url = "https://johannhof.github.io/text-diff.rs/")]

#![feature(step_by)]

// I can basically feel the karma already
#![deny(missing_docs)]

mod lcs;
mod merge;

use lcs::lcs;
use merge::merge;

/// Defines the contents of a changeset
/// Changesets will be delivered in order of appearance in the original string
/// Sequences of the same kind will be grouped into one Difference
#[derive(PartialEq, Debug)]
pub enum Difference {
    /// Sequences that are the same
    Same(String),
    /// Sequences that are an addition (don't appear in the first string)
    Add(String),
    /// Sequences that are a removal (don't appear in the second string)
    Rem(String)
}

/// Calculates the edit distance and the changeset for two given strings.
/// The first string is assumed to be the "original", the second to be an
/// edited version of the first. The third parameter specifies how to split
/// the input strings, leading to a more or less exact comparison.
///
/// Common splits are `""` for char-level, `" "` for word-level and `"\n"` for line-level.
///
/// Outputs the edit distance (how much the two strings differ) and a "changeset", that is
/// a `Vec` containing `Difference`s.
///
/// # Examples
///
/// ```
///
/// ```
pub fn diff(orig: &str, edit: &str, split: &str) -> (i32, Vec<Difference>) {
    let (dist, common) = lcs(orig, edit, split);
    (dist, merge(orig, edit, &common))
}

/// Prints a colorful visual representation of the diff.
/// This is just a convenience function for those who want quick results.
///
/// I recommend checking out the examples on how to build your
/// own diff output.
/// # Examples
///
/// ```
/// use text_diff::print_diff;
/// print_diff("Diffs are awesome", "Diffs are cool", " ");
/// ```
pub fn print_diff(orig: &str, edit: &str, split: &str) {
    let (_, changeset) = diff(orig, edit, split);
    let mut ret = String::new();

    for seq in changeset {
        match seq {
            Difference::Same(ref x) => {
                ret.push_str(x);
            },
            Difference::Add(ref x) => {
                ret.push_str("\x1B[92m");
                ret.push_str(x);
                ret.push_str("\x1B[0m");
            },
            Difference::Rem(ref x) => {
                ret.push_str("\x1B[91m");
                ret.push_str(x);
                ret.push_str("\x1B[0m");
            }
        }
    }
    println!("{}", ret);
}

