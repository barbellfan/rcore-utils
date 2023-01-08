/// Test byte counts using the -c command line argument.
mod test_utils;

#[cfg(test)]
mod test {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    
    use crate::test_utils as tu;

    /// Run wc with one small file and the -c switch:
    /// ```
    /// :~$ wc -c tests/test_files/trees.txt
    /// ```
    /// Output from wc looks like this:
    /// ```
    /// 21 tests/test_files/trees.txt
    /// ```
    /// Make the output look like that.
    #[test]
    fn test_read_bytes_trees() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = tu::get_cmd();

        cmd.arg("-c")
            .arg("tests/test_files/trees.txt")
            .assert()
            .success()
            .stdout(predicate::eq("415 tests/test_files/trees.txt\n"))
            .code(predicate::eq(0));

        Ok(())
    }

    /// Run wc with one big file, and one file with two words separated
    /// by and lots of empty lines.
    /// Count bytes only with the `-c` switch:
    /// ```
    /// :~$  tests/test_files/wc moby_dick.txt tests/test_files/bummer_dude.txt -c
    /// ```
    /// Output from wc looks like this:
    /// ```
    /// 1276231 tests/test_files/moby_dick.txt
    ///     110 tests/test_files/bummer_dude.txt
    /// 1276341 total
    /// ```
    /// Make the output look like that.
    #[test]
    fn read_moby_and_bummer_bytes() -> Result<(), Box<dyn std::error::Error>>{
        let expected = concat!(
            "1276231 tests/test_files/moby_dick.txt\n",
            "    110 tests/test_files/bummer_dude.txt\n",
            "1276341 total\n"
        );

        let mut cmd = tu::get_cmd();
        cmd.arg("tests/test_files/moby_dick.txt")
            .arg("tests/test_files/bummer_dude.txt")
            .arg("-c")
            .assert()
            .success()
            .stdout(predicate::eq(expected))
            .code(predicate::eq(0));
        
        Ok(())
    }
}
