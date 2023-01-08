/// Test using a combination of command line arguments.
mod test_utils;

#[cfg(test)]
mod test {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    
    use crate::test_utils as tu;

    /// Count characters and bytes in a small file where the byte and char counts
    /// are the same.
    /// ```
    /// :~$  wc -mc tests/test_files/so_tired_blues.txt
    /// ```
    /// Output from wc looks like this:
    /// ```
    /// 131 131 tests/test_files/so_tired_blues.txt
    /// ```
    /// Make the output look like that.
    #[test]
    fn chars_and_byte_count_same() -> Result<(), Box<dyn std::error::Error>> {
        let expected = "131 131 tests/test_files/so_tired_blues.txt\n";

        let mut cmd = tu::get_cmd();
        cmd.arg("tests/test_files/so_tired_blues.txt")
            .arg("-mc")
            .assert()
            .success()
            .code(predicate::eq(0))
            .stdout(predicate::eq(expected));

        Ok(())
    }

    /// Count characters and bytes in a large file where the byte and char counts
    /// are different.
    /// ```
    /// :~$  wc -mc tests/test_files/dracula.txt
    /// ```
    /// Output from wc looks like this:
    /// ```
    /// 881118 881220 tests/test_files/dracula.txt
    /// ```
    /// Make the output look like that.
    #[test]
    fn chars_and_byte_count_diff() -> Result<(), Box<dyn std::error::Error>> {
        let expected = "881118 881220 tests/test_files/dracula.txt\n";

        let mut cmd = tu::get_cmd();
        cmd.arg("tests/test_files/dracula.txt")
            .arg("-mc")
            .assert()
            .success()
            .code(predicate::eq(0))
            .stdout(predicate::eq(expected));

        Ok(())
    }
}
