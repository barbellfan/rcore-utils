//! Rust version of the classic Linux wc program.
//! 
//! Counts words, bytes, and lines from a file or from the pipeline.

use std::env::{current_exe, args};
use std::io::Error;

fn main() -> Result<(), Error> {
    let args : Vec<String> = args().collect();
    if args.len() == 1 {
        usage();
    } else {
        println!("args: {:?}", args);
    }

    Ok(())
}

fn usage() {
    let curr_ex = get_current_exe_name().unwrap();
    println!("current exe name: {}", curr_ex);
}

fn get_current_exe_name() -> Option<String> {
    current_exe()
        .ok()?
        .file_name()?
        .to_str()?
        .to_owned()
        .into()
}