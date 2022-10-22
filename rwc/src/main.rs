//! Rust version of the classic Linux wc program.
//! 
//! Counts words, bytes, and lines from a file or from the pipeline.

use std::env::{current_exe, args};
use std::io::Error;

use clap::{Parser};

mod wc; 

/// Usage: wc [OPTION]... [FILE]...
/// or:  wc [OPTION]... --files0-from=F
/// Print newline, word, and byte counts for each FILE, and a total line if
/// more than one FILE is specified.  A word is a non-zero-length sequence of
/// characters delimited by white space.
  
/// With no FILE, or when FILE is -, read standard input.
  
/// The options below may be used to select which counts are printed, always in
/// the following order: newline, word, character, byte, maximum line length.
/// -c, --bytes            print the byte counts
/// -m, --chars            print the character counts
/// -l, --lines            print the newline counts
///     --files0-from=F    read input from the files specified by
///                          NUL-terminated names in file F;
///                          If F is - then read names from standard input
/// -L, --max-line-length  print the maximum display width
/// -w, --words            print the word counts
///     --help     display this help and exit
///     --version  output version information and exit
///
/// GNU coreutils online help: <https://www.gnu.org/software/coreutils/>
/// Full documentation at: <https://www.gnu.org/software/coreutils/wc>
/// or available locally via: info '(coreutils) wc invocation'
/// 
/// This version is in Rust, and should act the same as the above.  
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short = 'l', long)]
    /// print the newline count
    lines: bool,
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
