use reqwest::Error;
use std::io::copy;
use std::fs::File;
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
    async fn fetch(&self, url: String, id: String) -> Result<(), reqwest::Error> {
        let content = reqwest::get(&url).await?.text().await?;
        self.copy_file(&content,id);
        Ok(())
    }

    fn copy_file(&self, content: &String, id: String) -> std::io::Result<()>{
        let dest_path = self.userpath.workshop_dir.join(id).join("download.zip");
        println!("dest_path: {}", dest_path.display());

        let mut dest_fp = File::create(dest_path)?;
        copy(&mut content.as_bytes(), &mut dest_fp)?;

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
        let id = parse_value.id;
        println!("fetch_url:{}", fetch_url);
        self.fetch(fetch_url, id);

        return Ok(String::from("OK"));
    }
}
