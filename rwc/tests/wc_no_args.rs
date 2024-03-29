/// Test using to command line arguments except for file names.
mod test_utils;

#[cfg(test)]
mod test {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    
    use crate::test_utils as tu;
    
    /// Run wc with one small file:
    /// ```
    /// :~$ wc tests/test_files/trees.txt
    /// ```
    /// Output from wc looks like this:
    /// ```
    ///  21  83 415 tests/test_files/trees.txt
    /// ```
    /// Make the output look like that.
    #[test]
    fn test_read_trees() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = tu::get_cmd();

        cmd.arg("tests/test_files/trees.txt")
            .assert()
            .success()
            .stdout(predicate::eq(" 21  83 415 tests/test_files/trees.txt\n"))
            .code(predicate::eq(0));

        Ok(())
    }

    /// Run wc with one small file:
    /// ```
    /// :~$ wc tests/test_files/fire_and_ice.txt
    /// ```
    /// Output from wc looks like this:
    /// ```
    ///  13  56 272 tests/test_files/fire_and_ice.txt
    /// ```
    /// Make the output look like that.
    #[test]
    fn read_fire() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = tu::get_cmd();

        cmd.arg("tests/test_files/fire_and_ice.txt")
            .assert()
            .success()
            .stdout(predicate::str::contains(" 13  56 272 tests/test_files/fire_and_ice.txt"))
            .code(predicate::eq(0));
        
        Ok(())
    }

    /// Run wc with one small file:
    /// ```
    /// :~$ wc tests/test_files/so_tired_blues.txt
    /// ```
    /// Output from wc looks like this:
    /// ```
    ///   9  26 131 tests/test_files/so_tired_blues.txt
    /// ```
    /// Make the output look like that.
    #[test]
    fn read_so_tired() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = tu::get_cmd();

        cmd.arg("tests/test_files/so_tired_blues.txt")
            .assert()
            .success()
            .stdout(predicate::eq("  9  26 131 tests/test_files/so_tired_blues.txt\n"))
            .code(predicate::eq(0));
        
        Ok(())
    }

    /// Run wc with two small files in this order:
    /// ```
    /// :~$ wc tests/test_files/so_tired_blues.txt tests/test_files/fire_and_ice.txt
    /// ```
    /// Output from wc looks like this:
    /// ```
    ///   9  26 131 tests/test_files/so_tired_blues.txt
    ///  13  56 272 tests/test_files/fire_and_ice.txt
    ///  22  82 403 total
    /// ```
    /// Make the output look like that.
    #[test]
    fn read_so_tired_and_fire() -> Result<(), Box<dyn std::error::Error>> {
        let expected = concat!(
            "  9  26 131 tests/test_files/so_tired_blues.txt\n",
            " 13  56 272 tests/test_files/fire_and_ice.txt\n",
            " 22  82 403 total\n") ;

        let mut cmd = tu::get_cmd();

        cmd.arg("tests/test_files/so_tired_blues.txt")
            .arg("tests/test_files/fire_and_ice.txt")
            .assert()
            .success()
            .stdout(predicate::eq(expected))
            .code(predicate::eq(0));
        
        Ok(())
    }

    /// Run wc with one big file:
    /// ```
    /// :~$ wc tests/test_files/dracula.txt
    /// ```
    /// Output from wc looks like this:
    /// ```
    ///  15857 164382 881220 tests/test_files/dracula.txt
    /// ```
    /// Make the output look like that.
    #[test]
    fn read_dracula() -> Result<(), Box<dyn std::error::Error>> {
        let expected = " 15857 164382 881220 tests/test_files/dracula.txt\n";

        let mut cmd = tu::get_cmd();
        cmd.arg("tests/test_files/dracula.txt")
            .assert()
            .success()
            .stdout(predicate::eq(expected))
            .code(predicate::eq(0));

        Ok(())
    }

    /// Run wc with one big file:
    /// ```
    /// :~$ wc tests/test_files/frankenstein.txt
    /// ```
    /// Output from wc looks like this:
    /// ```
    ///   7741  78122 448817 tests/test_files/frankenstein.txt
    /// ```
    /// Make the output look like that.
    #[test]
    fn read_frank() -> Result<(), Box<dyn std::error::Error>> {
        let expected = "  7741  78122 448817 tests/test_files/frankenstein.txt\n";

        let mut cmd = tu::get_cmd();
        cmd.arg("tests/test_files/frankenstein.txt")
            .assert()
            .success()
            .stdout(predicate::eq(expected))
            .code(predicate::eq(0));

        Ok(())
    }

    /// Run wc with one big file:
    /// ```
    /// :~$ wc tests/test_files/moby_dick.txt
    /// ```
    /// Output from wc looks like this:
    /// ```
    ///   22314  215864 1276231 tests/test_files/moby_dick.txt
    /// ```
    /// Make the output look like that.
    #[test]
    fn read_moby() -> Result<(), Box<dyn std::error::Error>> {
        let expected = "  22314  215864 1276231 tests/test_files/moby_dick.txt\n";

        let mut cmd = tu::get_cmd();
        cmd.arg("tests/test_files/moby_dick.txt")
            .assert()
            .success()
            .stdout(predicate::eq(expected))
            .code(predicate::eq(0));

        Ok(())
    }

    /// Run wc with three big files in this order:
    /// ```
    /// :~$ wc tests/test_files/moby_dick.txt tests/test_files/frankenstein.txt tests/test_files/dracula.txt
    /// ```
    /// Output from wc looks like this:
    /// ```
    ///   22314  215864 1276231 tests/test_files/moby_dick.txt
    ///    7741   78122  448817 tests/test_files/frankenstein.txt
    ///   15857  164382  881220 tests/test_files/dracula.txt
    ///   45912  458368 2606268 total
    /// ```
    /// Make the output look like that.
    #[test]
    fn read_3_big_files() -> Result<(), Box<dyn std::error::Error>> {
        let expected = concat!(
          "  22314  215864 1276231 tests/test_files/moby_dick.txt\n",
          "   7741   78122  448817 tests/test_files/frankenstein.txt\n",
          "  15857  164382  881220 tests/test_files/dracula.txt\n",
          "  45912  458368 2606268 total\n"
        );

        let mut cmd = tu::get_cmd();
        cmd.arg("tests/test_files/moby_dick.txt")
            .arg("tests/test_files/frankenstein.txt")
            .arg("tests/test_files/dracula.txt")
            .assert()
            .success()
            .stdout(predicate::eq(expected))
            .code(predicate::eq(0));

        Ok(())
    }
}