/// Test word counts using the -w command line argument.
mod test_utils;

#[cfg(test)]
mod test {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    
    use crate::test_utils as tu;


    /// Run wc with one small file and the -w switch:
    /// ```
    /// :~$ wc -w tests/test_files/trees.txt
    /// ```
    /// Output from wc looks like this:
    /// ```
    /// 21 tests/test_files/trees.txt
    /// ```
    /// Make the output look like that.
    #[test]
    fn test_read_words_trees() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = tu::get_cmd();

        cmd.arg("-w")
            .arg("tests/test_files/trees.txt")
            .assert()
            .success()
            .stdout(predicate::eq(" 83 tests/test_files/trees.txt\n"))
            .code(predicate::eq(0));

        Ok(())
    }

    /// Run wc with one big file, and one file with a single character and a bunch of empty lines.
    /// Count words only with the `-w` switch:
    /// ```
    /// :~$  wc tests/test_files/moby_dick.txt tests/test_files/one_char.txt -w
    /// ```
    /// Output from wc looks like this:
    /// ```
    ///  215864 tests/test_files/moby_dick.txt
    ///       1 tests/test_files/one_char.txt
    ///  215865 total
    /// ```
    /// Make the output look like that.
    #[test]
    fn read_moby_and_one_char() -> Result<(), Box<dyn std::error::Error>>{
        let expected = concat!(
            " 215864 tests/test_files/moby_dick.txt\n",
            "      1 tests/test_files/one_char.txt\n",
            " 215865 total\n"
        );

        let mut cmd = tu::get_cmd();
        cmd.arg("tests/test_files/moby_dick.txt")
            .arg("tests/test_files/one_char.txt")
            .arg("-w")
            .assert()
            .success()
            .stdout(predicate::eq(expected))
            .code(predicate::eq(0));
        
        Ok(())
    }
    
    /// Run wc with one big file, and one file with a really long line.
    /// Count words only with the `-w` switch:
    /// ```
    /// :~$  wc tests/test_files/moby_dick.txt tests/test_files/jack.txt -w
    /// ```
    /// Output from wc looks like this:
    /// ```
    ///  215864 tests/test_files/moby_dick.txt
    ///     240 tests/test_files/jack.txt
    ///  216104 total
    /// ```
    /// Make the output look like that.
    #[test]
    fn read_moby_and_jack() -> Result<(), Box<dyn std::error::Error>>{
        let expected = concat!(
            " 215864 tests/test_files/moby_dick.txt\n",
            "    240 tests/test_files/jack.txt\n",
            " 216104 total\n"
        );

        let mut cmd = tu::get_cmd();
        cmd.arg("tests/test_files/moby_dick.txt")
            .arg("tests/test_files/jack.txt")
            .arg("-w")
            .assert()
            .success()
            .stdout(predicate::eq(expected))
            .code(predicate::eq(0));
        
        Ok(())
    }
    
    /// Run wc with one big file, and one file with two words separated
    /// by and lots of empty lines.
    /// Count words only with the `-w` switch:
    /// ```
    /// :~$  wc tests/test_files/moby_dick.txt tests/test_files/bummer_dude.txt -w
    /// ```
    /// Output from wc looks like this:
    /// ```
    ///  215864 tests/test_files/moby_dick.txt
    ///     240 tests/test_files/bummer_dude.txt
    ///  215866 total
    /// ```
    /// Make the output look like that.
    #[test]
    fn read_moby_and_bummer_words() -> Result<(), Box<dyn std::error::Error>>{
        let expected = concat!(
            " 215864 tests/test_files/moby_dick.txt\n",
            "      2 tests/test_files/bummer_dude.txt\n",
            " 215866 total\n"
        );

        let mut cmd = tu::get_cmd();
        cmd.arg("tests/test_files/moby_dick.txt")
            .arg("tests/test_files/bummer_dude.txt")
            .arg("-w")
            .assert()
            .success()
            .stdout(predicate::eq(expected))
            .code(predicate::eq(0));
        
        Ok(())
    }
}
