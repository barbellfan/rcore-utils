
#[cfg(test)]
mod test {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;

    fn get_cmd() -> Command {
        Command::cargo_bin("wc").unwrap()
    }

    #[test]
    fn test_read_trees() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = get_cmd();

        cmd.arg("tests/test_files/trees.txt");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains(" 21  83 415 tests/test_files/trees.txt"))
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
            .stdout(predicate::str::contains(" 9  26 131 tests/test_files/so_tired_blues.txt"))
            .code(predicate::eq(0));
        
        Ok(())
    }

    #[test]
    fn read_so_tired_and_fire() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = get_cmd();

        cmd.arg("tests/test_files/so_tired_blues.txt")
            .arg("tests/test_files/fire_and_ice.txt");

        cmd.assert()
            .success()
            .stdout(predicate::str::contains(" 9  26 131 tests/test_files/so_tired_blues.txt"))
            .stdout(predicate::str::contains("13  56 272 tests/test_files/fire_and_ice.txt"))
            .stdout(predicate::str::contains("22  82 403 total"))
            .code(predicate::eq(0));
        
        Ok(())
    }
}
