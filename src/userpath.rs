use std::env;
use std::path::PathBuf;
use os_info;

pub struct UserPath{
    workshop_dir: PathBuf,
}

impl UserPath{
    pub fn new() -> Self{
        Self{
            workshop_dir: decide_workshop_dir(),
        }
    }

}

fn decide_workshop_dir()-> PathBuf{
    // os判別してsteamからインストールしたrimworldのworkshopのパスを決める
    let info = os_info::get();
    let os_type = info.os_type();

    let mut path = PathBuf::new();
    match os_type{
        os_info::Type::Windows => path.push(r"C:\"),
        os_info::Type::Macos => path.push("/Users/hogehoge/Library/Application Support/Steam/steamapps/workshop/content/"),
        _  => path.push("/home/"), // 未対応
    }

    // 参考: https://yajamon.hatenablog.com/entry/2018/03/05/201202
    //let path = env::home_dir().unwrap();
    println!("dir: {}", path.display());

    return path
}
