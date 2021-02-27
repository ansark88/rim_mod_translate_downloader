pub mod urlparser {
    use std::collections::HashMap;
    use url::Url;

    pub struct parse_value {
        pub converted_url: String,
        pub id: String,
    }

    // input例: https://rimworld.2game.info/jp_download.php?file_id=1214&id=2329126791
    // idとfile_idがkeyとなるHashMapを返す
    pub fn parse(input: String) -> Result<(parse_value), String> {
        let url = match Url::parse(&input) {
            Ok(url) => url,
            Err(_) => return Err(String::from("parse error")),
        };

        // 参考: https://stackoverflow.com/questions/43272935/how-do-i-decode-a-url-and-get-the-query-string-as-a-hashmap
        let hash_query: HashMap<_, _> = url.query_pairs().into_owned().collect();

        // hash_query.get() は Option<&v>なので&vをto_stringでvにする
        let id = match hash_query.get("id") {
            Some(id) => id.to_string(),
            None => return Err(String::from("\'id\' is None")),
        };

        let converted_url = convert(hash_query)?;

        return Ok(parse_value { converted_url, id });
    }

    // リンクのURLに対する実際のzipファイルのURLは以下のようになるので、変換する必要がある
    // https://img.2game.info/re_archive/l/rimworld/files/up_japanese/2205980094/935.zip
    fn convert(hash_query: HashMap<String, String>) -> Result<String, String> {
        let id = hash_query.get("id");
        let file_id = hash_query.get("file_id");

        if id == None || file_id == None {
            return Err(String::from("id or file_id is None"));
        }

        let converted_url = format!(
            "https://img.2game.info/re_archive/l/rimworld/files/up_japanese/{id}/{file_id}.zip",
            id = id.unwrap(),
            file_id = file_id.unwrap()
        );

        return Ok(converted_url);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use url::ParseError;

    #[test]
    fn test_parse_ok() {
        let value = urlparser::parse(
            "https://rimworld.2game.info/jp_download.php?file_id=1214&id=232912679".to_string()
        ).unwrap();

        let id = value.id;
        let converted_url = value.converted_url;

        assert_eq!(id, "232912679");
        assert_eq!(converted_url, "https://img.2game.info/re_archive/l/rimworld/files/up_japanese/232912679/1214.zip");
    }

    #[test]
    fn test_parse_ng() {
        assert_eq!(urlparser::parse("hogehoge".to_string()), Err("parse error"));
        // assert_ne!( urlparser::parse("https://rimworld.2game.info/jp_download.php?file_id=1214), Ok("232912679".to_string())); // panic起こすのでテスト不可
    }
}
