extern crate simian_council;

use ansi_term::Color;
use ansi_term::Color::Fixed;
use std::io::{self, prelude::*, StdoutLock};

fn main() {
    // Enable ANSI support for Windows
    /*
    #[cfg(windows)]
    let _ = ansi_term::enable_ansi_support();
    let stdout = io::stdout();

    writeln!(
        stdout.lock(),
        "┌{0:─<8}┬{0:─<25}┬{0:─<25}┬{0:─<8}┬{0:─<8}┐",
        ""
    )
    .ok();

    writeln!(
        stdout.lock(),
        "└{0:─<8}┴{0:─<25}┴{0:─<25}┴{0:─<8}┴{0:─<8}┘",
        ""
    )
    .ok();
    */

    let secret = simian_council::ask_for_a_secret(10, 15);
    println!("This is your secret : {}", secret);
}
