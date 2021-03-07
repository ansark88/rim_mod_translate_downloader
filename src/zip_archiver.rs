use std::fs;
use std::path::{Path, PathBuf};

pub struct ZipArchiver {
    zip_file: PathBuf,
}

impl ZipArchiver {
    pub fn new(zip_file: PathBuf) -> Self {
        Self { zip_file }
    }

    pub fn extract<P: AsRef<Path>>(&self, directory: P) -> Result<(), String> {
        let file = fs::File::open(&self.zip_file);
        match file {
            Ok(_) => (),
            Err(e) => return Err(format!("ZipFile Open Error!!!: {}", e.to_string())),
        }

        let archive = zip::ZipArchive::new(file.unwrap());
        match archive {
            Ok(_) => (),
            Err(e) => return Err(format!("ZipFile Read Error!!!: {}", e.to_string())),
        }

        let extract_result = archive.unwrap().extract(directory);
        match extract_result {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("ZipFile Extract Error!!!: {}", e.to_string())),
        }
    }
}
