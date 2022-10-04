
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

    #[test]
    fn read_dracula() -> Result<(), Box<dyn std::error::Error>> {
        let expected = " 15857 164382 881220 tests/test_files/dracula.txt\n";

        let mut cmd = get_cmd();
        cmd.arg("tests/test_files/dracula.txt");

        cmd.assert()
            .success()
            .stdout(predicate::eq(expected))
            .code(predicate::eq(0));

        Ok(())
    }

    #[test]
    fn read_frank() -> Result<(), Box<dyn std::error::Error>> {
        let expected = "  7741  78122 448817 tests/test_files/frankenstein.txt\n";

        let mut cmd = get_cmd();
        cmd.arg("tests/test_files/frankenstein.txt");

        cmd.assert()
            .success()
            .stdout(predicate::eq(expected))
            .code(predicate::eq(0));

        Ok(())
    }

    #[test]
    fn read_moby() -> Result<(), Box<dyn std::error::Error>> {
        let expected = "  22314  215864 1276231 tests/test_files/moby_dick.txt\n";

        let mut cmd = get_cmd();
        cmd.arg("tests/test_files/moby_dick.txt");

        cmd.assert()
            .success()
            .stdout(predicate::eq(expected))
            .code(predicate::eq(0));

        Ok(())
    }

    #[test]
    fn read_err() -> Result<(), Box<dyn std::error::Error>> {
        let expected_linux = "No such file or directory (os error 2): tests/test_files/does_not_exist.txt\n";
        let expected_windows = "The system cannot find the file specified. (os error 2): tests/test_files/does_not_exist.txt\n";
        let expected = match std::env::consts::OS {
            "linux" => expected_linux,
            "windows" => expected_windows,
            _ => panic!("Not tested on this operating system: {}", std::env::consts::OS),
        };

        let mut cmd = get_cmd();
        cmd.arg("tests/test_files/does_not_exist.txt");

        cmd.assert()
            .success()
            .stderr(predicate::eq(expected));
        
        Ok(())
    }

    #[test]
    fn read_3_big_files() -> Result<(), Box<dyn std::error::Error>> {
        let expected = concat!(
          "  22314  215864 1276231 tests/test_files/moby_dick.txt\n",
          "   7741   78122  448817 tests/test_files/frankenstein.txt\n",
          "  15857  164382  881220 tests/test_files/dracula.txt\n",
          "  45912  458368 2606268 total\n"
        );

        let mut cmd = get_cmd();
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
