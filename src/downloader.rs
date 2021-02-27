use reqwest::Error;
use std::fs;
use std::io;
use tempfile::Builder;

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

    // 参考 https://rust-lang-nursery.github.io/rust-cookbook/web/clients/download.html
    #[tokio::main]
    async fn fetch(&self, url: String, id: String) -> Result<(), Error> {
        let dist_dir = self.userpath.workshop_dir.join(id);
        println!("dist_dir: {}", dist_dir.display());

        //let tmp_dir = Builder::new().prefix("tmp").tempdir()?;
        let content = reqwest::get(&url).await?.text().await?;
        println!("{}", content);
        Ok(())
    }

    pub fn download(&self) -> Result<String, String> {
        let parse_url = self.url.to_string();
        let parse_result = urlparser::parse(parse_url);

        let parse_value = match parse_result {
            Ok(v) => v,
            Err(e) => return Err(e.to_string()),
        };

        let fetch_url = parse_value.converted_url;
        println!("fetch_url:{}", fetch_url);
        //self.fetch(fetch_url, id);

        return Ok(String::from("OK"));
    }
}
