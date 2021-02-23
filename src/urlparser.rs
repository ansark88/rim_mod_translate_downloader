pub mod urlparser {
    use std::collections::HashMap;
    use url::{ParseError, Url};

    // input例: https://rimworld.2game.info/jp_download.php?file_id=1214&id=2329126791
    pub fn parse(input: String) -> Result<String, ParseError> {
        let url = Url::parse(&input)?;

        // 参考: https://stackoverflow.com/questions/43272935/how-do-i-decode-a-url-and-get-the-query-string-as-a-hashmap
        let hash_query: HashMap<_, _> = url.query_pairs().into_owned().collect();
        let id = hash_query.get("id").unwrap(); // Noneの時のハンドリングができてない...

        return Ok(id.to_string()); // hash_query.get() は Option<&v>なので&vをto_stringでvにする
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use url::{ParseError, Url};

    #[test]
    fn test_parse_ok() {
        assert_eq!(
            urlparser::parse(
                "https://rimworld.2game.info/jp_download.php?file_id=1214&id=232912679".to_string()
            ),
            Ok("232912679".to_string())
        );
    }

    #[test]
    fn test_parse_ng() {
        assert_eq!(
            urlparser::parse("hogehoge".to_string()),
            Err(ParseError::RelativeUrlWithoutBase)
        );
        // assert_ne!( urlparser::parse("https://rimworld.2game.info/jp_download.php?file_id=1214), Ok("232912679".to_string())); // panic起こすのでテスト不可
    }
}
