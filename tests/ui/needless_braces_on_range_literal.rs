// run-rustfix
// edition:2018

#![warn(clippy::needless_braces_on_range_literal)]
#![allow(clippy::almost_complete_letter_range)]

fn main() {
    let _ = ('a')..=('z');
}
