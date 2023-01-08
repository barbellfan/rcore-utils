/// Test character counts using the -m command line switch.
mod test_utils;

#[cfg(test)]
mod test {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    
    use crate::test_utils as tu;

    /// Count characters in one big file using the -m switch.
    /// ```
    /// :~$  wc -m tests/test_files/frankenstein.txt
    /// ```
    /// I'm picking this file because the byte count char counts are different.
    /// 
    /// Output from wc looks like this:
    /// ```
    /// 446616 tests/test_files/frankenstein.txt
    /// ```
    /// Make the output look like that.
    #[test]
    fn count_chars_frank() -> Result<(), Box<dyn std::error::Error>> {
        let expected = "446616 tests/test_files/frankenstein.txt\n";

        let mut cmd = tu::get_cmd();
        cmd.arg("tests/test_files/frankenstein.txt")
            .arg("-m")
            .assert()
            .success()
            .code(predicate::eq(0))
            .stdout(predicate::eq(expected));
        
        Ok(())
    }

    /// Count characters in two big files using the -m switch.
    /// ```
    /// :~$  wc -m tests/test_files/frankenstein.txt tests/test_files/dracula.txt
    /// ```
    /// I'm picking these files because the byte and char counts are different.
    /// 
    /// Output from wc looks like this:
    /// ```
    ///  446616 tests/test_files/frankenstein.txt
    ///  881118 tests/test_files/dracula.txt
    /// 1327734 total
    /// ```
    /// Make the output look like that.
    #[test]
    fn count_chars_frank_n_drac() -> Result<(), Box<dyn std::error::Error>> {
        let expected = concat!(
            " 446616 tests/test_files/frankenstein.txt\n",
            " 881118 tests/test_files/dracula.txt\n",
            "1327734 total\n"
        );

        let mut cmd = tu::get_cmd();
        cmd.arg("tests/test_files/frankenstein.txt")
            .arg("tests/test_files/dracula.txt")
            .arg("-m")
            .assert()
            .success()
            .code(predicate::eq(0))
            .stdout(predicate::eq(expected));
        
        Ok(())
    }
}
