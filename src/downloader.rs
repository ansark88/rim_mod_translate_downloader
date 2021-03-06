use bytes::Buf;
use std::fs::File;
use std::io::copy;
use std::path::PathBuf;
use thiserror::Error;

use crate::urlparser::urlparser;
use crate::userpath::UserPath;

#[derive(Error, Debug)]
enum MyError {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    IO(#[from] std::io::Error),
}

pub struct Downloader {
    user_path: UserPath,
    url: String,
    dest_path: PathBuf,
}

impl Downloader {
    pub fn new(user_path: UserPath, url: String) -> Self {
        let dest_path = PathBuf::new();
        Self {
            user_path,
            url,
            dest_path,
        }
    }

    // 参考 https://rust-lang-nursery.github.io/rust-cookbook/web/clients/download.html
    #[tokio::main]
    async fn fetch(&mut self, url: String, id: String) -> Result<(), MyError> {
        let client = reqwest::Client::builder()
            .user_agent(format!("custom-user-agent"))
            .build()?;

        let content = client
            .get(&url)
            .send()
            .await
            .map_err(MyError::from)?
            .bytes()
            .await
            .map_err(MyError::from)?;

        let reader = content.reader(); // std::io::Readを実装しているトレイトを渡す必要があり、reader()メソッドでBufトレイトを返している
        self.copy_file(reader, id).map_err(MyError::from)?;
        Ok(())
    }

    fn copy_file<R: std::io::Read>(&mut self, mut reader: R, id: String) -> std::io::Result<()> {
        let dest_path = self.user_path.workshop_dir.join(id).join("download.zip");
        println!("dest_path: {}", dest_path.display());

        self.dest_path = dest_path.clone();

        let mut dest_fp = File::create(dest_path)?;
        copy(&mut reader, &mut dest_fp)?;

        Ok(())
    }

    // ダウンロード成功時はダウンロード先のパスを返す
    pub fn download(&mut self) -> Result<PathBuf, String> {
        let parse_url = self.url.to_string();

        let parse_result = urlparser::parse(parse_url);
        let parse_value = match parse_result {
            Ok(v) => v,
            Err(e) => return Err(e),
        };

        let fetch_url = parse_value.converted_url;
        let id = parse_value.id;
        println!("fetch_url:{}", fetch_url);

        let fetch_result = self.fetch(fetch_url, id);
        match fetch_result {
            Ok(_) => Ok(self.dest_path.clone()),
            Err(e) => Err(e.to_string()),
        }
    }
}
