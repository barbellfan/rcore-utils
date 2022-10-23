//! Rust version of the classic Linux wc program.
//! 
//! Counts words, bytes, and lines from a file or from the pipeline.

use std::env::{current_exe, args};
use std::io::Error;

use clap::{Parser};

mod wc; 

/// thing
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short = 'l', long)]
    /// print the newline count
    lines: bool,

    #[arg(short = 'c', long)]
    /// print the byte counts
    bytes: bool,

    #[arg(short = 'm', long)]
    /// print the character counts
    chars: bool,

    #[arg(short = 'L', long = "max-line-length")]
    /// print the maximum display width
    max_line_length: Option<i32>,

    #[arg(short = 'w', long)]
    /// print the word counts
    words: bool,

    
}

/// Entry point for the program.
fn main() -> Result<(), Error> {
    let clap_args = Cli::parse();
    println!("lines: {}", clap_args.lines);

    let args : Vec<String> = args().collect();
    if args.len() == 1 { // should always be length 1 if no args given
        usage();
        return Ok(())
    }

    wc::wc(args)
}

/// Display usage directions. Should be the same as or very
/// similar to the standard wc command.
fn usage() {
    let curr_ex = get_current_exe_name().unwrap();
    println!("current exe name: {}", curr_ex);
}

/// Get the name of the current executable. By default,
/// it will be wc (wc.exe on Windows). Used by the usage()
/// function for display.
/// 
/// During tests, the expected exe name has some sort of hash
/// appended to it. Must be something weird with Rust tests.
/// Not sure if that hash will ever change.
fn get_current_exe_name() -> Option<String> {
    current_exe()
        .ok()?
        .file_name()?
        .to_str()?
        .to_owned()
        .into()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore = "does weird stuff"]
    fn exe_name() {
        // the exe name is weird when doing the test.
        match std::env::consts::OS {
            "linux" => assert_eq!("wc-08a2a4111f7a35e5".to_owned(), get_current_exe_name().unwrap()),
            "windows" => assert_eq!("wc-188f7b7b1d75f60c.exe".to_owned(), get_current_exe_name().unwrap()),
            _ => panic!("Not tested on this operating system: {}", std::env::consts::OS),
        }
    }
}
