/// Test error handling.
mod test_utils;

#[cfg(test)]
mod test {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    
    use crate::test_utils as tu;


    /// Run wc with a file that does not exist, like this:
    /// ```
    /// :~$ wc tests/test_files/does_not_exist.txt
    /// ```
    /// Output from wc looks like this:
    /// ```
    /// wc: tests/test_files/does_not_exist.txt: No such file or directory
    /// ```
    /// Make the output look like that, depending on the OS.
    #[test]
    fn read_err() -> Result<(), Box<dyn std::error::Error>> {
        let expected_linux = "No such file or directory (os error 2): tests/test_files/does_not_exist.txt\n";
        let expected_windows = "The system cannot find the file specified. (os error 2): tests/test_files/does_not_exist.txt\n";
        let expected = match std::env::consts::OS {
            "linux" => expected_linux,
            "windows" => expected_windows,
            _ => panic!("Not tested on this operating system: {}", std::env::consts::OS),
        };

        let mut cmd = tu::get_cmd();
        cmd.arg("tests/test_files/does_not_exist.txt")
            .assert()
            .success()
            .stderr(predicate::eq(expected))
            .code(predicate::eq(0));
        
        Ok(())
    }
}
