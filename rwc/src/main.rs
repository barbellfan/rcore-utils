//! Rust version of the classic Linux wc program.
//! 
//! Counts words, bytes, and lines from a file or from the pipeline.

use std::env::{current_exe};
use std::io::Error;

use clap::{Parser};

mod wc; 

/// Struct that contains information about the command line options that were entered.
/// Used by the `clap` library.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short = 'l', long)]
    /// Print the newline count
    lines: bool,

    #[arg(short = 'c', long)]
    /// Print the byte counts
    bytes: bool,

    #[arg(short = 'm', long)]
    /// Print the character counts
    chars: bool,

    #[arg(short = 'w', long)]
    /// Print the word counts
    words: bool,

    #[arg(short = 'L', long = "max-line-length")]
    /// Print the maximum display width
    max_line_length: Option<i32>,

    /// List of files to process
    files: Option<Vec<String>>,
}

/// Entry point for the program.
fn main() -> Result<(), Error> {
    let mut clap_args = Cli::parse();

    // if all are set to false, then none were set on the command line
    // set all but chars to true
    if clap_args.lines == false 
        && clap_args.bytes == false 
        && clap_args.words == false
        && clap_args.chars == false {
        clap_args.lines = true;
        clap_args.bytes = true;
        clap_args.words = true;
    }

    if let Some(_) = clap_args.files {
        wc::wc(clap_args)
    } else {
        usage();
        return Ok(())
    }
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
/* Invalid test. In a testing context, the executable name has what looks like a hash appended to it.
The hash changes when you change the code.
Unless there's a better way to test this, don't bother.
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
*/
