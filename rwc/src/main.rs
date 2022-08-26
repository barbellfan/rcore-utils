//! Rust version of the classic Linux wc program.
//! 
//! Counts words, bytes, and lines from a file or from the pipeline.

use std::env::{current_exe, args};
use std::io::Error;

/// Entry point for the program.
fn main() -> Result<(), Error> {
    let args : Vec<String> = args().collect();
    if args.len() == 1 {
        usage();
    } else {
        println!("args: {:?}", args);
    }

    Ok(())
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