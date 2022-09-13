
#[cfg(test)]
mod test {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;

    #[test]
    fn test_read_trees() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("wc")?;

        cmd.arg("tests/test_files/trees.txt");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains(" 21  83 415 tests/test_files/trees.txt"));


        Ok(())
    }
}
