//! Rust version of the classic Linux wc program.
//! 
//! Counts words, bytes, and lines from a file or from the pipeline.

use std::env::{current_exe, args};
use std::io::Error;
use std::fs;

/// Entry point for the program.
fn main() -> Result<(), Error> {
    let args : Vec<String> = args().collect();
    if args.len() == 1 { // should always be length 1 if no args given
        usage();
    } else {
        let file_path = &args[1];
        let contents = fs::read_to_string(file_path);
        match contents {
            Ok(c) => {
                let mut summary = handle_file_contents(c);
                summary.file_name = file_path.to_owned();
                print_summary(summary);
            },
            Err(e) => println!("error: {:?}", e),
        };

        //println!("contents of file:\n{}", contents);
    }

    Ok(())
}

#[derive(Debug)]
struct FileSummary {
    lines: usize,
    words: usize,
    chars: usize,
    file_name: String,
}

fn print_summary(summary: FileSummary) {
    println!("{}\t{}\t{}\t{}", summary.lines, summary.words, summary.chars, summary.file_name);
}

fn handle_file_contents(contents: String) -> FileSummary {
    let mut fs = FileSummary { 
        lines: 0, 
        words: 0, 
        chars: 0, 
        file_name: "".to_owned()
    };

    fs.lines = contents.lines().count();
    fs.words = contents.split_ascii_whitespace().count();
    fs.chars = contents.len();

    fs
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
fn get_current_exe_name() -> Option<String> {
    current_exe()
        .ok()?
        .file_name()?
        .to_str()?
        .to_owned()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_file_contents_1() {
        let simple_str = "this is a short bit of text".to_owned();
        let fs = handle_file_contents(simple_str);
        assert_eq!(fs.lines, 1);
        assert_eq!(fs.words, 7);
        assert_eq!(fs.chars, 27);
    }
}