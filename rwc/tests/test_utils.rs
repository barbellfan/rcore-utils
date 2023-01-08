use std::process::Command;

use assert_cmd::prelude::CommandCargoExt;

pub fn get_cmd() -> Command {
    Command::cargo_bin("wc").unwrap()
}
