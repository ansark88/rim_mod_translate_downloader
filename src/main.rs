// TODO
// DONE:引数を取る(URL)
// DONE:引数をパースして適切なダウンロード先を決める
// DONE:ダウンロードする
// ダウンロードしたファイルを解凍して配置する
// 複数URL対応
// テスト書く
use clap::{App, Arg};

mod downloader;
mod urlparser;
mod userpath;

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

    let userpath = userpath::UserPath::new();

    let downloader = downloader::Downloader::new(userpath, url.unwrap().to_string());

    match downloader.download() {
        Ok(_) => println!("Download Complete!"),
        Err(e) => println!("Download Error!!! {}", e),
    }
}
