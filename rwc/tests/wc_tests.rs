
#[cfg(test)]
mod test {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;

    fn get_cmd() -> Command {
        Command::cargo_bin("wc").unwrap()
    }

    // try using predicate::eq for the stdout strings
    // concat strings as needed to build up multiline output

    #[test]
    fn test_read_trees() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = get_cmd();

        cmd.arg("tests/test_files/trees.txt");
        cmd.assert()
            .success()
            .stdout(predicate::eq(" 21  83 415 tests/test_files/trees.txt\n"))
            .code(predicate::eq(0));

        Ok(())
    }

    #[test]
    fn read_fire() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = get_cmd();

        cmd.arg("tests/test_files/fire_and_ice.txt");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains(" 13  56 272 tests/test_files/fire_and_ice.txt"))
            .code(predicate::eq(0));
        
        Ok(())
    }

    #[test]
    fn read_so_tired() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = get_cmd();

        cmd.arg("tests/test_files/so_tired_blues.txt");
        cmd.assert()
            .success()
            .stdout(predicate::eq("  9  26 131 tests/test_files/so_tired_blues.txt\n"))
            .code(predicate::eq(0));
        
        Ok(())
    }

    #[test]
    fn read_so_tired_and_fire() -> Result<(), Box<dyn std::error::Error>> {
        let expected = concat!(
            "  9  26 131 tests/test_files/so_tired_blues.txt\n",
            " 13  56 272 tests/test_files/fire_and_ice.txt\n",
            " 22  82 403 total\n") ;

        let mut cmd = get_cmd();

        cmd.arg("tests/test_files/so_tired_blues.txt")
            .arg("tests/test_files/fire_and_ice.txt");
        
        cmd.assert()
            .success()
            .stdout(predicate::eq(expected))
            .code(predicate::eq(0));
        
        Ok(())
    }
}
