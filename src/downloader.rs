use std::fs;
use std::io;

use crate::urlparser::urlparser;
use crate::userpath::UserPath;


pub struct Downloader {
    userpath: UserPath,
    url: String,
}

impl Downloader {
    pub fn new(userpath: UserPath, url: String) -> Self {
        Self {
            userpath: userpath,
            url: url,
        }
    }

    pub fn download(&self) -> Result<String, String> {
        let url = self.url.to_string();

        let parse_result = urlparser::parse(url);
        match parse_result {
            Ok(id) => Ok(id),
            Err(e) => {
                eprintln!("{}", e);
                return Err(String::from("parse error"));
            }
        }
    }
}
