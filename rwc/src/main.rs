//! Rust version of the classic Linux wc program.
//! 
//! Counts words, bytes, and lines from a file or from the pipeline.

use std::env::{current_exe, args};
//use std::error::Error;
use std::io::Error;
use std::fs;
use std::cmp::max;

/// Entry point for the program.
fn main() -> Result<(), Error> {
    let args : Vec<String> = args().collect();
    if args.len() == 1 { // should always be length 1 if no args given
        usage();
        return Ok(())
    }

    let mut summaries: Vec<FileSummary> = Vec::new();
    let mut file_errors: Vec<String> = Vec::new();

    let file_names = &args[1..];
    for file_path in file_names.iter() {
        let contents = fs::read_to_string(file_path);
        match contents {
            Ok(c) => {
                let mut summary = handle_file_contents(c);
                summary.label = file_path.to_owned();
                summaries.push(summary);
            },
            Err(e) => file_errors.push(format!("{}: {}", e.to_string(), file_path)),
        };
    }

    // get longest number so you can set the amount of padding
    // also get a running total of all lines, words, and chars
    let mut max_len = 0;
    let mut total_summary = FileSummary {
        lines: 0,
        words: 0,
        chars: 0,
        label: "total".to_owned(),
    };

    for file_summary in summaries.iter() {
        // make totals
        total_summary.lines += file_summary.lines;
        total_summary.words += file_summary.words;
        total_summary.chars += file_summary.chars;

        // get longest number
        max_len = max(max_len, file_summary.lines.to_string().len());
        max_len = max(max_len, file_summary.words.to_string().len());
        max_len = max(max_len, file_summary.chars.to_string().len());
    }
    if summaries.len() > 1 {
        summaries.push(total_summary);
    }

    for fs in summaries.iter() {
        print_summary(fs, max_len);
    }

    for fe in file_errors.iter() {
        println!("Error: {}", fe);
    }

    Ok(())
}

/// Struct that contains info about the files that wc is told to get info about.
#[derive(Debug)]
struct FileSummary {
    /// Number of lines found in the file
    lines: usize,
    /// Number of words found in the file.
    words: usize, 
    /// Number of characters found in the file.
    chars: usize, 
    /// Label for thing being counted. Is either the file name or the total.
    label: String, 
}

/// TO DO
/// Print a file summary to standard out like the original wc command.
/// 
/// This means the following:
/// * Calculate the value with the longest number of chars, and
/// pad to that length.
/// * Then separate each value by one character.
/// * Right justify the numbers.
/// 
/// If there is more than one file, check the length of each file's
/// values. The format! macro has justify and padding options, and include
/// a total line at the end.
/// 
/// This makes a nice output like this:
/// ```
/// :~$ wc .xsession-errors .xsession-errors.old .xinputrc
///    87   627  7695 .xsession-errors
///   118   881 10564 .xsession-errors.old
///     3    17   131 .xinputrc
///   208  1525 18390 total
/// ```
/// For missing files, write the output like this:
/// ```
/// :~$ wc .xsession-errors .xsession-errors.old .xinpur
///    87   627  7695 .xsession-errors
///   118   881 10564 .xsession-errors.old
/// wc: .xinputr: No such file or directory
///   208  1525 18390 total
/// ```
/// # Arguments
/// 
/// * `summary` - a FileSummary object (soon to be a collection of them) containing
/// the files to print to std out.
fn print_summary(summary: &FileSummary, padding: usize) {
    println!("{:>padding$} {:>padding$} {:>padding$} {:>padding$}", summary.lines, summary.words, summary.chars, summary.label);
}

fn handle_file_contents(contents: String) -> FileSummary {
    let mut fs = FileSummary { 
        lines: 0, 
        words: 0, 
        chars: 0, 
        label: "".to_owned()
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