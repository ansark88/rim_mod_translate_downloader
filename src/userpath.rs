use std::env;
use std::path::PathBuf;

pub struct UserPath {
    pub workshop_dir: PathBuf,
}

impl UserPath {
    pub fn new() -> Self {
        Self {
            workshop_dir: decide_workshop_dir(),
        }
    }
}

fn decide_workshop_dir() -> PathBuf {
    // os判別してsteamからインストールしたrimworldのworkshopのパスを決める
    let info = os_info::get();
    let os_type = info.os_type();

    // 参考: https://yajamon.hatenablog.com/entry/2018/03/05/201202
    let home_dir = env::home_dir().unwrap();

    let mut path: PathBuf;
    match os_type {
        os_info::Type::Windows => {
            path = PathBuf::from(r"C:\Program Files (x86)\Steam\steamapps\workshop\content\294100")
        }
        os_info::Type::Macos => {
            path = home_dir;
            path.push("Library/Application Support/Steam/steamapps/workshop/content/294100")
        }
        _ => path = PathBuf::from("/hoge/"), // 未対応
    };

    println!("dir: {}", path.display());

    path
}
