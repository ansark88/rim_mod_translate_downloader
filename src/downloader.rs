use std::fs;
use std::io;

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

    pub fn download(&self) -> io::Result<String> {
        Ok(String::from("OK"))
    }
}
