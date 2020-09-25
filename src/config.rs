use std::fs::File;

use serde::{Deserialize, Serialize};
use serde_yaml;

/// Config struct.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub subject: String,
    pub sender_addr: String,
    pub sender_name: String,
    pub smtp_srv: String,
    pub smtp_port: u64,
    pub smtp_usr: String,
    pub smtp_pwd: String,
    pub recipients: Vec<String>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            subject: "Some Message Subject".to_string(),
            sender_addr: "info@domain.com".to_string(),
            sender_name: "Domain".to_string(),
            smtp_srv: "smtp.domain.com".to_string(),
            smtp_port: 465,
            smtp_usr: "info@domain.com".to_string(),
            smtp_pwd: "secret".to_string(),
            recipients: vec![],
        }
    }

    pub fn open<S: Into<String>>(path: S) -> Self {
        let pth = path.into();
        let file = match File::open(&pth) {
            Ok(x) => x,
            Err(e) => panic!("{}", e),
        };
        match serde_yaml::from_reader(file) {
            Ok(x) => x,
            Err(e) => panic!("{}", e),
        }
    }

    pub fn save<S: Into<String>>(&self, path: S) {
        let file = match File::create(&path.into()) {
            Ok(x) => x,
            Err(e) => panic!("{}", e),
        };
        match serde_yaml::to_writer(file, self) {
            Err(e) => panic!("{}", e),
            _ => {}
        }
    }

    pub fn recipients_tuple(&self) -> Vec<(String, String)> {
        self.recipients.iter().map(|&x| (x, String::from(""))).collect()
    }
}
