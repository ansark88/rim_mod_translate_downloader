// TODO
// 引数を取る(URL)
// 引数をパースして適切なダウンロード先を決める
// ダウンロードしたファイルを解凍して配置する
// テスト書く
use clap::{App, Arg};

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

    match matches.value_of("url"){
        Some(url) => println!("input url: {}", url),
        None => println!("No input url!"),
    }

    let userpath = userpath::UserPath::new();
}


