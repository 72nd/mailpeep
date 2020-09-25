use crate::config;

use std::fs;

use lettre_email::Email;

pub fn send<S: Into<String>>(cfg: &config::Config, msg_path: S) {
    let msg = fs::read_to_string(msg_path.into()).expect("error reading message file");

    let email = Email::builder()
        .from(cfg.sender_addr.clone())
        .to((cfg.sender_addr.clone(), cfg.sender_name.clone()))
        .bcc(cfg.recipients_tuple());
}

