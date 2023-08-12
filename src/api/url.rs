use crate::error::TankerkoenigError;
use reqwest::Url;

pub(crate) trait TankerUrl {
    fn base_url(&self, api_key: &str, with_path: Option<&str>) -> Result<Url, TankerkoenigError> {
        let mut url = Url::parse("https://creativecommons.tankerkoenig.de/")
            .map_err(|_| TankerkoenigError::UrlConstruction)?;
        url.query_pairs_mut().append_pair("apikey", api_key);
        if let Some(path) = with_path {
            url.set_path(path);
        }
        Ok(url)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct TestStruct {}
    impl TankerUrl for TestStruct {}

    #[test]
    fn should_create_base_url_with_api_key() {
        let api = TestStruct {};

        let base_url = api.base_url("123", None).unwrap();
        assert_eq!(base_url.query(), Some("apikey=123"));
    }

    #[test]
    fn should_create_base_url() {
        let api = TestStruct {};
        let base_url = api.base_url("123", None).unwrap();
        assert_eq!(base_url.host_str(), Some("creativecommons.tankerkoenig.de"));
    }

    #[test]
    fn should_create_base_url_with_path() {
        let api = TestStruct {};
        let base_url = api.base_url("123", Some("json/list.php")).unwrap();
        assert_eq!(base_url.path(), "/json/list.php");
    }
}
