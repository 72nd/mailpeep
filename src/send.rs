use crate::config;

use std::fs;

pub fn send<S: Into<String>>(cfg: &config::Config, msg_path: S) {
    let msg = fs::read_to_string(msg_path.into()).expect("error reading message file");
}

