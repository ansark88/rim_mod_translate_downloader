// TODO
// DONE:引数を取る(URL)
// DONE:引数をパースして適切なダウンロード先を決める
// DONE:ダウンロードする
// ダウンロードしたファイルを解凍して配置する
// 複数URL対応
// テスト書く
use clap::{App, Arg};
use std::path::Path;

mod downloader;
mod urlparser;
mod userpath;
mod zip_archiver;

fn main() {
    let matches = App::new("rmtd")
        .version("1.0")
        .author("ansark88")
        .about("Download the Japanese translation patch for the Rimworld Mod to the appropriate filepath")
        .arg(Arg::new("url")
            .about("Sets the download website URL")
            .value_name("URL")
            .required(true)
            .index(1))
        .get_matches();

    let url = matches.value_of("url");

    match url {
        Some(url) => println!("input url: {}", url),
        None => println!("No input url!"),
    }

    // ダウンロード処理
    let userpath = userpath::UserPath::new();
    let mut downloader = downloader::Downloader::new(userpath, url.unwrap().to_string());
    let download_result = downloader.download();

    match download_result {
        Ok(_) => println!("Download Complete!"),
        Err(e) => return println!("Download Error!!! {}", e),
    }

    // Zip解凍処理
    let dest_path = download_result.unwrap();
    let directory = dest_path.parent().unwrap_or_else(|| Path::new("/"));
    let zip_archiver = zip_archiver::ZipArchiver::new(dest_path.clone());

    // Zipファイルと同じディレクトリに解凍する
    let extract_result = zip_archiver.extract(directory);

    match extract_result {
        Ok(_) => println!("Complete!!!"),
        Err(e) => return println!("Error Exit!!! {}", e),
    }
}
