/// Test line counts using the -l command line switch.
mod test_utils;

#[cfg(test)]
mod test {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    
    use crate::test_utils as tu;

    /// Run wc with one small file and the -l switch:
    /// ```
    /// :~$ wc -l tests/test_files/trees.txt
    /// ```
    /// Output from wc looks like this:
    /// ```
    ///  21 tests/test_files/trees.txt
    /// ```
    /// Make the output look like that.
    #[test]
    fn test_read_lines_trees() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = tu::get_cmd();

        cmd.arg("-l")
            .arg("tests/test_files/trees.txt")
            .assert()
            .success()
            .stdout(predicate::eq(" 21 tests/test_files/trees.txt\n"))
            .code(predicate::eq(0));

        Ok(())
    }


    /// Run wc with two small files in this order:
    /// ```
    /// :~$ wc -l tests/test_files/so_tired_blues.txt tests/test_files/fire_and_ice.txt
    /// ```
    /// Output from wc looks like this:
    /// ```
    ///   9 tests/test_files/so_tired_blues.txt
    /// 13 tests/test_files/fire_and_ice.txt
    /// 22 total
    /// ```
    /// Make the output look like that.
    #[test]
    fn read_so_tired_and_fire_lines() -> Result<(), Box<dyn std::error::Error>> {
        let expected = concat!(
            "  9 tests/test_files/so_tired_blues.txt\n",
            " 13 tests/test_files/fire_and_ice.txt\n",
            " 22 total\n") ;

        let mut cmd = tu::get_cmd();

        cmd.arg("-l")
            .arg("tests/test_files/so_tired_blues.txt")
            .arg("tests/test_files/fire_and_ice.txt")
            .assert()
            .success()
            .stdout(predicate::eq(expected))
            .code(predicate::eq(0));
        
        Ok(())
    }
}
