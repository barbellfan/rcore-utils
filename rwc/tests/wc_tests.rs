
#[cfg(test)]
mod test {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;

    fn get_cmd() -> Command {
        Command::cargo_bin("wc").unwrap()
    }

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
        let mut cmd = get_cmd();

        cmd.arg("tests/test_files/trees.txt");
        cmd.assert()
            .success()
            .stdout(predicate::eq(" 21  83 415 tests/test_files/trees.txt\n"))
            .code(predicate::eq(0));

        Ok(())
    }

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
        let mut cmd = get_cmd();

        cmd.arg("-l")
            .arg("tests/test_files/trees.txt");
            
        cmd.assert()
            .success()
            .stdout(predicate::eq(" 21 tests/test_files/trees.txt\n"))
            .code(predicate::eq(0));

        Ok(())
    }

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
        let mut cmd = get_cmd();

        cmd.arg("-c")
            .arg("tests/test_files/trees.txt");
            
        cmd.assert()
            .success()
            .stdout(predicate::eq("415 tests/test_files/trees.txt\n"))
            .code(predicate::eq(0));

        Ok(())
    }

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
        let mut cmd = get_cmd();

        cmd.arg("-w")
            .arg("tests/test_files/trees.txt");
            
        cmd.assert()
            .success()
            .stdout(predicate::eq(" 83 tests/test_files/trees.txt\n"))
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
        let mut cmd = get_cmd();

        cmd.arg("tests/test_files/fire_and_ice.txt");
        cmd.assert()
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
        let mut cmd = get_cmd();

        cmd.arg("tests/test_files/so_tired_blues.txt");
        cmd.assert()
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

        let mut cmd = get_cmd();

        cmd.arg("tests/test_files/so_tired_blues.txt")
            .arg("tests/test_files/fire_and_ice.txt");
        
        cmd.assert()
            .success()
            .stdout(predicate::eq(expected))
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

        let mut cmd = get_cmd();

        cmd.arg("-l")
            .arg("tests/test_files/so_tired_blues.txt")
            .arg("tests/test_files/fire_and_ice.txt");
        
        cmd.assert()
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

        let mut cmd = get_cmd();
        cmd.arg("tests/test_files/dracula.txt");

        cmd.assert()
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

        let mut cmd = get_cmd();
        cmd.arg("tests/test_files/frankenstein.txt");

        cmd.assert()
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

        let mut cmd = get_cmd();
        cmd.arg("tests/test_files/moby_dick.txt");

        cmd.assert()
            .success()
            .stdout(predicate::eq(expected))
            .code(predicate::eq(0));

        Ok(())
    }

    /// Run wc with a file that does not exist, like this:
    /// ```
    /// :~$ wc tests/test_files/does_not_exist.txt
    /// ```
    /// Output from wc looks like this:
    /// ```
    /// wc: tests/test_files/does_not_exist.txt: No such file or directory
    /// ```
    /// Make the output look like that, depending on the OS.
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

        let mut cmd = get_cmd();
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

        let mut cmd = get_cmd();
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

        let mut cmd = get_cmd();
        cmd.arg("tests/test_files/moby_dick.txt")
            .arg("tests/test_files/bummer_dude.txt")
            .arg("-w")
            .assert()
            .success()
            .stdout(predicate::eq(expected))
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

        let mut cmd = get_cmd();
        cmd.arg("tests/test_files/moby_dick.txt")
            .arg("tests/test_files/bummer_dude.txt")
            .arg("-c")
            .assert()
            .success()
            .stdout(predicate::eq(expected))
            .code(predicate::eq(0));
        
        Ok(())
    }

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

        let mut cmd = get_cmd();
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

        let mut cmd = get_cmd();
        cmd.arg("tests/test_files/frankenstein.txt")
            .arg("tests/test_files/dracula.txt")
            .arg("-m")
            .assert()
            .success()
            .code(predicate::eq(0))
            .stdout(predicate::eq(expected));
        
        Ok(())
    }

    /// Count characters in a small file where the byte and char counts
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

        let mut cmd = get_cmd();
        cmd.arg("tests/test_files/so_tired_blues.txt")
            .arg("-mc")
            .assert()
            .success()
            .code(predicate::eq(0))
            .stdout(predicate::eq(expected));

        Ok(())
    }

    /// Count characters in a large file where the byte and char counts
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

        let mut cmd = get_cmd();
        cmd.arg("tests/test_files/dracula.txt")
            .arg("-mc")
            .assert()
            .success()
            .code(predicate::eq(0))
            .stdout(predicate::eq(expected));

        Ok(())
    }
}
