use std::io::Error;
use std::fs;
use std::cmp::max;

use crate::Cli;

/// Enum that handles the two cases that wc can run up against: a file, or an error message.
enum WCResult {
    FileStats(FileSummary),
    ErrMsg(String),
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
    /// Number of bytes found in the file.
    bytes: usize, 
    /// Label for thing being counted. Is either the file name or `total`.
    label: String, 
}

/// Count words, lines, and bytes in the given files.
pub(crate) fn wc(args: Cli) -> Result<(), Error> {

    if let Some(file_names) = &args.files {
        let mut summaries = summarize_files(&file_names);

        let max_len = get_totals(&mut summaries);

        summaries.iter().for_each(|file_summary_result| {
            match file_summary_result {
                WCResult::FileStats(s) => println!("{}", format_summary(s, max_len, &args)),
                WCResult::ErrMsg(e) => eprintln!("{}", e),
            };
        });

        Ok(())
    } else {
        Ok(())
    }
}

/// Get totals of all files, if there is more than one.
/// 
/// Returns a `usize` containing the length of the longest number
/// in all of the structs. Longest meaning the largest number of digits.
/// This is used as padding later.
/// 
/// Get the longest number regardless of command line arguments. 
/// It looks like the standard wc does this, so duplicate the behavior.
/// 
/// # Arguments
/// 
///  * `summaries` - A Vec of `WCResult` enums. If there is more than 
/// one, add a `FileSummary` struct with the label "total" 
/// at the end. This will contain totals of all the other structs.
fn get_totals(summaries: &mut Vec<WCResult>) -> usize {
    // get longest number so you can set the amount of padding
    // also get a running total of all lines, words, and chars
    let mut max_len = 0;
    let mut total_summary = FileSummary {
        lines: 0,
        words: 0,
        chars: 0,
        bytes: 0,
        label: "total".to_owned(),
    };

    for file_summary_result in summaries.iter() {
        if let WCResult::FileStats(filsm) = file_summary_result {
            // calculate totals if there is more than one file
            if summaries.len() > 1 {
                total_summary.lines += filsm.lines;
                total_summary.words += filsm.words;
                total_summary.chars += filsm.chars;
                total_summary.bytes += filsm.bytes;
            }

            // get longest number
            max_len = max(max_len, filsm.lines.to_string().len());
            max_len = max(max_len, filsm.words.to_string().len());
            max_len = max(max_len, filsm.chars.to_string().len());
            max_len = max(max_len, filsm.bytes.to_string().len());
        }
    }

    if summaries.len() > 1 {
        // max len might be longer here if other totals make longer numbers
        max_len = max(max_len, total_summary.lines.to_string().len());
        max_len = max(max_len, total_summary.words.to_string().len());
        max_len = max(max_len, total_summary.chars.to_string().len());
        max_len = max(max_len, total_summary.bytes.to_string().len());

        summaries.push(WCResult::FileStats(total_summary));
    }

    max_len
}


/// Take a list of files and summarize them.
/// 
/// Return a Vec of `WCResult` enums, which can either be a `FileSummary`
/// struct, or a `String` which should be an error message.
/// 
/// # Arguments
/// 
/// * `file_names` - a pointer to an array of Strings that are file names 
/// recieved from the user at the command line.
fn summarize_files(file_names: &Vec<String>) -> Vec<WCResult> {
    let mut summaries: Vec<WCResult> = Vec::new();

    for file_path in file_names.iter() {
        let contents = fs::read_to_string(file_path);
        match contents {
            Ok(c) => {
                let mut summary = handle_file_contents(c);
                summary.label = file_path.to_owned();
                summaries.push(WCResult::FileStats(summary));
            },
            Err(e) => summaries.push(WCResult::ErrMsg(format!("{}: {}", e.to_string(), file_path))),
        };
    }

    summaries
}

/// Format a `FileSummary` struct to look like the original wc command's output.
/// 
/// This means the following:
/// * Calculate the value with the longest number of chars, and
/// pad to that length.
/// * Then separate each value by one character.
/// * Right justify the numbers.
/// 
/// If there is more than one file, check the length of each file's
/// values. The `format!` macro has justify and padding options, and include
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
/// List all items in the order they were specified on the command line,
/// including any errors. This means that the error may be in the middle of the list.
/// 
/// # Arguments
/// 
/// * `summary` - a `WCResult` enum that can contain a `FileSummary` struct, or an 
/// error message as a String.
/// * `padding` - the number of spaces to pad between values on a line. Get this by
/// looping through all of the `FileSummary` structs and getting the largest value, 
/// meaning the longest number when converted to a `String`.
/// * `args` - the command line arguments, as a reference to a `Cli` object
fn format_summary(f: &FileSummary, padding: usize, args: &Cli) -> String {
    let mut lines_count = "".to_owned();
    let mut words_count = "".to_owned();
    let mut chars_count = "".to_owned();
    let mut bytes_count = "".to_owned();

    if args.lines {
        lines_count = format!("{:>padding$} ", f.lines);
    }
    if args.words {
        words_count = format!("{:>padding$} ", f.words);
    }
    if args.chars {
        chars_count = format!("{:>padding$} ", f.chars);
    }
    if args.bytes {
        bytes_count = format!("{:>padding$} ", f.bytes);
    }
    format!("{}{}{}{}{}", lines_count, words_count, chars_count, bytes_count, f.label)
}

/// Utility function to count lines, words, and bytes in the given file. Return a 
/// `FileSummary` struct.
/// # Arguments
/// * `contents` - the contents of the file in question, as a `String`.
fn handle_file_contents(contents: String) -> FileSummary {
    let mut fs = FileSummary { 
        lines: 0, 
        words: 0,
        chars: 0,
        bytes: 0, 
        label: "".to_owned()
    };

    // The lines() function checks for either \n or \r\n. Final line ending is optional.
    // So a file ending in an empty line is the same as one with no final line ending.
    // See rust docs for core::str::lines().
    fs.lines = contents.lines().count();
    fs.words = contents.split_ascii_whitespace().count();
    fs.chars = contents.chars().count();
    fs.bytes = contents.len();
    
    fs
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper method to make debugging easier.
    /// 
    /// Rust debugging in a workspace sets the working directory differently
    /// from just running a test without debugging. This makes it hard to debug
    /// tests, because the test cannot find the files it needs to test.
    /// 
    /// Fix by checking whether test is debugging, by checking whether the
    /// current directory is `rwc`. Tests like this will be different for other 
    /// programs in this workspace.
    /// 
    /// There's no need to call this if the test does not read files directly
    fn debug_set_working_dir() {
        if !std::env::current_dir().unwrap().ends_with("rwc") {
            std::env::set_current_dir("rwc").unwrap();
        }
    }

    /// Helper method to create default input from the user. Default means no
    /// special arguments, which means they want to show line, byte, and word
    /// counts. Tests can adjust as needed.
    fn get_default_args() -> Cli {
        Cli {
            lines: true,
            bytes: true,
            chars: false,
            words: true,
            max_line_length: None,
            files: None
        }
    }

    fn check_file_summary_val(num_found: usize, num_expected: usize, val_type: String) {
        assert_eq!(num_found, num_expected,
            "wc should have counted {} {}(s), but found {}",
            num_expected,
            val_type,
            num_found);
    }

    #[test]
    /// Simple test to make sure handle_file_contents counts words and stuff.
    fn test_handle_file_contents_1() {
        let simple_str = "this is a short bit of text".to_owned();
        let fs = handle_file_contents(simple_str);
        check_file_summary_val(fs.lines, 1, "line".to_owned());
        check_file_summary_val(fs.words, 7, "word".to_owned());
        check_file_summary_val(fs.bytes, 27, "byte".to_owned());
    }

    #[test]
    /// Read the file trees.txt and get various counts for it.
    fn read_trees() {
        debug_set_working_dir();
        let file_sum = summarize_files(&vec!["tests/test_files/trees.txt".to_owned()]);
        assert_eq!(file_sum.len(), 1); // there should be just one item in this vec.

        match &file_sum[0] {
            WCResult::FileStats(fs) => {
                check_file_summary_val(fs.lines, 21, "line".to_owned());
                check_file_summary_val(fs.words, 83, "word".to_owned());
                check_file_summary_val(fs.bytes, 415, "byte".to_owned());
            },
            WCResult::ErrMsg(e) => {
                panic!("Should not have caused an error: {}", e);
            }
        }
    }

    /// Read the file fire_and_ice.txt and get various counts for it.
    #[test]
    fn read_fire() {
        debug_set_working_dir();
        let file_sum = summarize_files(&vec!["tests/test_files/fire_and_ice.txt".to_owned()]);
        assert_eq!(file_sum.len(), 1); // there should be just one item in this vec.

        match &file_sum[0] {
            WCResult::FileStats(fs) => {
                check_file_summary_val(fs.lines, 13, "line".to_owned());
                check_file_summary_val(fs.words, 56, "word".to_owned());
                check_file_summary_val(fs.bytes, 272, "byte".to_owned());
            },
            WCResult::ErrMsg(e) => {
                panic!("Should not have caused an error: {}", e);
            }
        }
    }

    /// Read the file so_tired_blues.txt and get various counts for it.
    #[test]
    fn read_so_tired() {
        debug_set_working_dir();
        let file_sum = summarize_files(&vec!["tests/test_files/so_tired_blues.txt".to_owned()]);
        assert_eq!(file_sum.len(), 1); // there should be just one item in this vec.

        match &file_sum[0] {
            WCResult::FileStats(fs) => {
                check_file_summary_val(fs.lines, 9, "line".to_owned());
                check_file_summary_val(fs.words, 26, "word".to_owned());
                check_file_summary_val(fs.bytes, 131, "byte".to_owned());
            },
            WCResult::ErrMsg(e) => {
                panic!("Should not have caused an error: {}", e);
            }
        }
    }

    /// Read two files and get their counts. Check the totals.
    #[test]
    fn read_so_tired_and_fire() {
        debug_set_working_dir();
        let args = get_default_args();
        let mut file_sum = summarize_files(
            &vec![
                "tests/test_files/so_tired_blues.txt".to_owned(),
                "tests/test_files/fire_and_ice.txt".to_owned()
            ]);

        assert_eq!(file_sum.len(), 2); // there should be two items in this vec.
        // Both entries in vec should be FileStats enums
        assert!(matches!(file_sum[0], WCResult::FileStats(_)));
        assert!(matches!(file_sum[1], WCResult::FileStats(_)));

        match &file_sum[0] {
            WCResult::FileStats(fs) => {
                check_file_summary_val(fs.lines, 9, "line".to_owned());
                check_file_summary_val(fs.words, 26, "word".to_owned());
                check_file_summary_val(fs.bytes, 131, "byte".to_owned());
            },
            WCResult::ErrMsg(e) => {
                panic!("Should not have caused an error: {}", e);
            }
        }

        match &file_sum[1] {
            WCResult::FileStats(fs) => {
                check_file_summary_val(fs.lines, 13, "line".to_owned());
                check_file_summary_val(fs.words, 56, "word".to_owned());
                check_file_summary_val(fs.bytes, 272, "byte".to_owned());
            },
            WCResult::ErrMsg(e) => {
                panic!("Should not have caused an error: {}", e);
            }
        }

        let max_len = get_totals(&mut file_sum);
        assert_eq!(max_len, 3, "max length used for padding should be 3");
        assert_eq!(file_sum.len(), 3, "vec should have 3 item in it now");

        match &file_sum[0] {
            WCResult::FileStats(fs)=> {
                let expected_so_tired_blues = "  9  26 131 tests/test_files/so_tired_blues.txt";
                let found_so_tired_blues = format_summary(fs, max_len, &args);
                assert_eq!(found_so_tired_blues, expected_so_tired_blues, "Output not correct");
            },
            WCResult::ErrMsg(e) => {
                panic!("Should not have caused an error: {}", e);
            }
        }
        
        match &file_sum[1] {
            WCResult::FileStats(fs)=> {
                let expected_fire_and_ice = " 13  56 272 tests/test_files/fire_and_ice.txt";
                let found_fire_and_ice = format_summary(fs, max_len, &args);
                assert_eq!(found_fire_and_ice, expected_fire_and_ice, "Output not correct");
            },
            WCResult::ErrMsg(e) => {
                panic!("Should not have caused an error: {}", e);
            }
        }
        
        match &file_sum[2] {
            WCResult::FileStats(fs)=> {
                let expected_total = " 22  82 403 total";
                let found_total = format_summary(fs, max_len, &args);
                assert_eq!(found_total, expected_total, "Output not correct");
            },
            WCResult::ErrMsg(e) => {
                panic!("Should not have caused an error: {}", e);
            }
        }
        
    }

    /// Read two files, but this time, just count the lines. Check the totals.
    #[test]
    fn read_so_tired_and_fire_lines() {
        debug_set_working_dir();
        let mut args = get_default_args();
        args.bytes = false;
        args.chars = false;
        args.words = false;

        let mut file_sum = summarize_files(
            &vec![
            "tests/test_files/so_tired_blues.txt".to_owned(),
            "tests/test_files/fire_and_ice.txt".to_owned()
            ]);

        assert_eq!(file_sum.len(), 2); // there should be two items in this vec.
        // Both entries in vec should be FileStats enums
        assert!(matches!(file_sum[0], WCResult::FileStats(_)));
        assert!(matches!(file_sum[1], WCResult::FileStats(_)));

        match &file_sum[0] {
            WCResult::FileStats(fs) => {
                check_file_summary_val(fs.lines, 9, "line".to_owned());
                check_file_summary_val(fs.words, 26, "word".to_owned());
                check_file_summary_val(fs.bytes, 131, "byte".to_owned());
            },
            WCResult::ErrMsg(e) => {
                panic!("Should not have caused an error: {}", e);
            }
        }

        match &file_sum[1] {
            WCResult::FileStats(fs) => {
                check_file_summary_val(fs.lines, 13, "line".to_owned());
                check_file_summary_val(fs.words, 56, "word".to_owned());
                check_file_summary_val(fs.bytes, 272, "byte".to_owned());
            },
            WCResult::ErrMsg(e) => {
                panic!("Should not have caused an error: {}", e);
            }
        }
        let max_len = get_totals(&mut file_sum);
        assert_eq!(max_len, 3, "max length used for padding should be 3");
        assert_eq!(file_sum.len(), 3, "vec should have 3 item in it now");

        match &file_sum[0] {
            WCResult::FileStats(fs)=> {
                let expected_so_tired_blues = "  9 tests/test_files/so_tired_blues.txt";
                let found_so_tired_blues = format_summary(fs, max_len, &args);
                assert_eq!(found_so_tired_blues, expected_so_tired_blues, "Output not correct");
            },
            WCResult::ErrMsg(e) => {
                panic!("Should not have caused an error: {}", e);
            }
        }
        /**/
        
        match &file_sum[1] {
            WCResult::FileStats(fs)=> {
                let expected_fire_and_ice = " 13 tests/test_files/fire_and_ice.txt";
                let found_fire_and_ice = format_summary(fs, max_len, &args);
                assert_eq!(found_fire_and_ice, expected_fire_and_ice, "Output not correct");
            },
            WCResult::ErrMsg(e) => {
                panic!("Should not have caused an error: {}", e);
            }
        }
        
        match &file_sum[2] {
            WCResult::FileStats(fs)=> {
                let expected_total = " 22 total";
                let found_total = format_summary(fs, max_len, &args);
                assert_eq!(found_total, expected_total, "Output not correct");
            },
            WCResult::ErrMsg(e) => {
                panic!("Should not have caused an error: {}", e);
            }
        }
        /**/
    }

    /// Read dracula.txt, a larger file.
    #[test]
    fn read_dracula() {
        debug_set_working_dir();
        let file_sum = summarize_files(&vec!["tests/test_files/dracula.txt".to_owned()]);
        assert_eq!(file_sum.len(), 1); // there should be just one item in this vec.

        match &file_sum[0] {
            WCResult::FileStats(fs) => {
                check_file_summary_val(fs.lines, 15857, "line".to_owned());
                check_file_summary_val(fs.words, 164382, "word".to_owned());
                check_file_summary_val(fs.bytes, 881220, "byte".to_owned());
            },
            WCResult::ErrMsg(e) => {
                panic!("Should not have caused an error: {}", e);
            }
        }
    }

    /// Read another big file, frankenstein.txt.
    #[test]
    fn read_frank() {
        debug_set_working_dir();
        let file_sum = summarize_files(&vec!["tests/test_files/frankenstein.txt".to_owned()]);
        assert_eq!(file_sum.len(), 1); // there should be just one item in this vec.

        match &file_sum[0] {
            WCResult::FileStats(fs) => {
                check_file_summary_val(fs.lines, 7741, "line".to_owned());
                check_file_summary_val(fs.words, 78122, "word".to_owned());
                check_file_summary_val(fs.bytes, 448817, "byte".to_owned());
            },
            WCResult::ErrMsg(e) => {
                panic!("Should not have caused an error: {}", e);
            }
        }
    }

    /// Read a third big file, moby_dick.txt.
    #[test]
    fn read_moby() {
        debug_set_working_dir();
        let file_sum = summarize_files(&vec!["tests/test_files/moby_dick.txt".to_owned()]);
        assert_eq!(file_sum.len(), 1); // there should be just one item in this vec.

        match &file_sum[0] {
            WCResult::FileStats(fs) => {
                check_file_summary_val(fs.lines, 22314, "line".to_owned());
                check_file_summary_val(fs.words, 215864, "word".to_owned());
                check_file_summary_val(fs.bytes, 1276231, "byte".to_owned());
            },
            WCResult::ErrMsg(e) => {
                panic!("Should not have caused an error: {}", e);
            }
        }
    }

    /// Try reading a file that does not exist. Output depends on the OS. Hopefully that does not change much 
    /// between OS versions.
    #[test]
    fn read_err() {
        debug_set_working_dir();
        let file_sum = summarize_files(&vec!["tests/test_files/does_not_exist.txt".to_owned()]);
        assert_eq!(file_sum.len(), 1); // there should be just one item in this vec.

        match &file_sum[0] {
            WCResult::FileStats(_) => {
                panic!("Should not have found the file");
            },
            WCResult::ErrMsg(e) => {
                match std::env::consts::OS {
                    "linux" => assert_eq!(e, "No such file or directory (os error 2): tests/test_files/does_not_exist.txt"),
                    "windows" => assert_eq!(e, "The system cannot find the file specified. (os error 2): src/wc/test_files/does_not_exist.txt"),
                    _ => panic!("Not tested on this operating system: {}", std::env::consts::OS),
                };
            }
        }
    }

    /// Just test the get_totals() function with mock structs.
    #[test]
    fn test_get_totals() {
        let f1 = FileSummary {lines: 1, words: 1, chars: 1, bytes: 1, label: "file_1".to_owned()};
        let f2 = FileSummary {lines: 2, words: 2, chars: 1, bytes: 2, label: "file_2".to_owned()};

        let mut fv = vec!();
        fv.push(WCResult::FileStats(f1));
        fv.push(WCResult::FileStats(f2));

        let max_len = get_totals(&mut fv);
        assert_eq!(max_len, 1, "Max length for padding should have been {}, but was {}", max_len, 1);

        assert_eq!(fv.len(), 3, "get_totals should have added one item to the vec. Expected length of 3, but found {}", fv.len());

        match &fv[2] {
            WCResult::FileStats(fs) => {
                check_file_summary_val(fs.lines, 3, "line".to_owned());
                check_file_summary_val(fs.words, 3, "word".to_owned());
                check_file_summary_val(fs.bytes, 3, "byte".to_owned());
                assert_eq!(fs.label, "total".to_owned());
            },
            WCResult::ErrMsg(e) => {
                panic!("Should not have caused this error: {}", e);
            }
        }
    }

    /// Just test the format_summary() function with mock structs, and command line arguments equal to -l.
    #[test]
    fn test_get_format_summary_lines() {
        let f1 = FileSummary {lines: 1, words: 11, chars: 111, bytes: 11111, label: "file_1".to_owned()};
        let f2 = FileSummary {lines: 22, words: 2, chars: 1, bytes: 2, label: "file_2".to_owned()};
        let mut args = get_default_args();
        args.lines = true;
        args.words = false;
        args.chars = false;
        args.bytes = false;

        let f1_expected = "  1 file_1";
        let sum1 = format_summary(&f1, 3, &args);
        assert_eq!(sum1, f1_expected);

        let f2_expected = " 22 file_2";
        let sum2 = format_summary(&f2, 3, &args);
        assert_eq!(sum2, f2_expected);
    }

    /// Test whether there is a totals line if you only read one file.
    #[test]
    fn test_no_totals_with_one_file() {
        let f1 = FileSummary {lines: 1, words: 1, chars: 1, bytes: 1, label: "file_1".to_owned()};

        let mut fv = vec!();
        fv.push(WCResult::FileStats(f1));

        get_totals(&mut fv);

        assert_eq!(fv.len(), 1, "get_totals should NOT have added one item to the vec since there was only one item. Expected length of 1, but found {}", fv.len());

        match &fv[0] {
            WCResult::FileStats(fs) => {
                check_file_summary_val(fs.lines, 1, "line".to_owned());
                check_file_summary_val(fs.words, 1, "word".to_owned());
                check_file_summary_val(fs.bytes, 1, "byte".to_owned());
                assert_eq!(fs.label, "file_1".to_owned());
            },
            WCResult::ErrMsg(e) => {
                panic!("Should not have caused this error: {}", e);
            }
        }
    }

    /// Test missing file output when the first one in the argument is the one that's missing.
    #[test]
    fn read_err_2() {
        debug_set_working_dir();
        let file_sum = summarize_files(
            &vec![
                "tests/test_files/does_not_exist.txt".to_owned(),
                "tests/test_files/moby_dick.txt".to_owned()
                ]);
        assert_eq!(file_sum.len(), 2); // there should be just one item in this vec.

        match &file_sum[0] {
            WCResult::FileStats(_) => {
                panic!("Should not have found the file");
            },
            WCResult::ErrMsg(e) => {
                let expected_linux = "No such file or directory (os error 2): tests/test_files/does_not_exist.txt";
                let expected_windows = "The system cannot find the file specified. (os error 2): src/wc/test_files/does_not_exist.txt";
                let expected = match std::env::consts::OS {
                    "linux" => expected_linux,
                    "windows" => expected_windows,
                    _ => panic!("Not tested on this operating system: {}", std::env::consts::OS),
                };

                assert_eq!(e, expected);
            }
        }

        match &file_sum[1] {
            WCResult::FileStats(fs) => {
                check_file_summary_val(fs.lines, 22314, "line".to_owned());
                check_file_summary_val(fs.words, 215864, "word".to_owned());
                check_file_summary_val(fs.bytes, 1276231, "byte".to_owned());
            },
            WCResult::ErrMsg(e) => {
                panic!("Should not have caused an error: {}", e);
            }
        }
    }

    /// Test missing file output when the second one out of three arguments is the one that's missing.
    #[test]
    fn read_err_3() {
        debug_set_working_dir();
        let file_sum = summarize_files(
            &vec![
            "tests/test_files/frankenstein.txt".to_owned(),
            "tests/test_files/does_not_exist.txt".to_owned(),
            "tests/test_files/moby_dick.txt".to_owned()
            ]);
        assert_eq!(file_sum.len(), 3); // there should be just one item in this vec.

        match &file_sum[0] {
            WCResult::FileStats(fs) => {
                check_file_summary_val(fs.lines, 7741, "line".to_owned());
                check_file_summary_val(fs.words, 78122, "word".to_owned());
                check_file_summary_val(fs.bytes, 448817, "byte".to_owned());
            },
            WCResult::ErrMsg(e) => {
                panic!("Should not have caused an error: {}", e);
            }
        }
        
        match &file_sum[1] {
            WCResult::FileStats(_) => {
                panic!("Should not have found the file");
            },
            WCResult::ErrMsg(e) => {
                let expected_linux = "No such file or directory (os error 2): tests/test_files/does_not_exist.txt";
                let expected_windows = "The system cannot find the file specified. (os error 2): src/wc/test_files/does_not_exist.txt";
                let expected = match std::env::consts::OS {
                    "linux" => expected_linux,
                    "windows" => expected_windows,
                    _ => panic!("Not tested on this operating system: {}", std::env::consts::OS),
                };
                assert_eq!(e, expected);
            }
        }

        match &file_sum[2] {
            WCResult::FileStats(fs) => {
                check_file_summary_val(fs.lines, 22314, "line".to_owned());
                check_file_summary_val(fs.words, 215864, "word".to_owned());
                check_file_summary_val(fs.bytes, 1276231, "byte".to_owned());
            },
            WCResult::ErrMsg(e) => {
                panic!("Should not have caused an error: {}", e);
            }
        }
    }

    /// Test setting the padding parameter to 5 using a mock struct.
    #[test]
    fn test_format_summary_padding_5() {
        let ws = FileSummary{lines: 1, words: 1, chars: 1, bytes: 1, label: "thing".to_owned()};
        let args = get_default_args();
        let s = format_summary(&ws, 5, &args);
        assert_eq!(s, "    1     1     1 thing");
    }

    /// Test setting the padding parameter to 2 using a mock struct.
    #[test]
    fn test_format_summary_padding_2() {
        let ws = FileSummary{lines: 1, words: 1, chars: 1, bytes: 1, label: "thing".to_owned()};
        let args = get_default_args();
        let s = format_summary(&ws, 2, &args);
        assert_eq!(s, " 1  1  1 thing");
    }

    /// Test the padding size when reading several large files.
    #[test]
    fn test_format_summary_padding_3() {
        debug_set_working_dir();
        let mut file_sum = summarize_files(
            &vec![
            "tests/test_files/dracula.txt".to_owned(),
            "tests/test_files/frankenstein.txt".to_owned()]);
        let max_len = get_totals(&mut file_sum);
        assert_eq!(max_len, 7, "Max length should have been 7, but was {}", max_len);
    }
}
