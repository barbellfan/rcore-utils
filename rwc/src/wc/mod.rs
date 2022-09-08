use std::io::Error;
use std::fs;
use std::cmp::max;

/// Count words, lines, and bytes in the given files.
pub fn wc(args: Vec<String>) -> Result<(), Error> {

    let file_names = &args[1..];
    let (mut summaries, file_errors) = summarize_files(file_names);

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

/// Take a list of files and summarize them.
/// 
/// Return a tuple containing a Vec of FileSummary structs, and a Vec of error Strings.
/// 
/// # Arguments
/// 
/// * `file_names` - a pointer to an array of Strings that are file names recieved from the user at the command line.
fn summarize_files(file_names: &[String]) -> (Vec<FileSummary>, Vec<String>) {
    let mut summaries: Vec<FileSummary> = Vec::new();
    let mut file_errors: Vec<String> = Vec::new();

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

    (summaries, file_errors)
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
/// * `padding` - the number of spaces to pad between values on a line. Get this by
/// looping through all of the FileSummary objects and getting the largest value, 
/// meaning the longest number when converted to a String.
fn print_summary(summary: &FileSummary, padding: usize) {
    println!("{:>padding$} {:>padding$} {:>padding$} {:>padding$}", summary.lines, summary.words, summary.chars, summary.label);
}

/// Utility function to count lines, words, and characters in the given file. Save to a FileSummary struct.
/// # Arguments
/// * `contents` - the contents of the file in question, as a String.
fn handle_file_contents(contents: String) -> FileSummary {
    let mut fs = FileSummary { 
        lines: 0, 
        words: 0, 
        chars: 0, 
        label: "".to_owned()
    };

    // The lines() function checks for either \n or \r\n. Final line ending is optional.
    // So a file ending in an empty line is the same as one with no final line ending.
    // See rust docs for core::str::lines().
    fs.lines = contents.lines().count();
    fs.words = contents.split_ascii_whitespace().count();
    fs.chars = contents.len();

    fs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Simple test to make sure handle_file_contents counts words and stuff.
    fn test_handle_file_contents_1() {
        let simple_str = "this is a short bit of text".to_owned();
        let fs = handle_file_contents(simple_str);
        assert_eq!(fs.lines, 1);
        assert_eq!(fs.words, 7);
        assert_eq!(fs.chars, 27);
    }

    #[test]
    // Read the file trees.txt and get various counts for it.
    fn read_trees() {
        let (file_sum, file_err) = summarize_files(&["src/wc/test_files/trees.txt".to_owned()]);
        assert_eq!(file_sum.len(), 1); // there should be just one item in this vec.
        assert_eq!(file_err.len(), 0); // no items should be here.

        let trees_sum = &file_sum[0];
        assert_eq!(trees_sum.lines, 21);
        assert_eq!(trees_sum.words, 83);
        assert_eq!(trees_sum.chars, 415);
    }
}
