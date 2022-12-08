pub(crate) mod baseline {
    use crate::api;
    use crate::error::TankerkoenigError;
    use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
    use reqwest::{Client, Url};

    const BASE_URL: &str = "https://creativecommons.tankerkoenig.de/";

    pub fn construct_client(
        user_agent: Option<&str>,
        settings: &api::Settings,
    ) -> Result<Client, TankerkoenigError> {
        let mut headers = HeaderMap::new();
        let content_type = HeaderValue::from_str("application/json")?;
        headers.insert(CONTENT_TYPE, content_type);
        let agent = user_agent.map_or_else(
            || format!("{}/{}", settings.package_name, settings.package_version),
            String::from,
        );
        let client = Client::builder()
            .user_agent(agent)
            .default_headers(headers)
            .build()
            .map_err(|err| TankerkoenigError::ClientConstruction { source: err })?;
        Ok(client)
    }

    pub fn construct_base_url(
        api_key: &str,
        with_path: Option<&str>,
    ) -> Result<Url, TankerkoenigError> {
        let mut url = Url::parse(BASE_URL).map_err(|_| TankerkoenigError::UrlConstruction)?;
        url.query_pairs_mut().append_pair("apikey", api_key);
        if let Some(path) = with_path {
            url.set_path(path);
        }
        Ok(url)
    }
}

pub(crate) mod price {
    use std::{cell::RefCell, rc::Rc};

    pub fn format_ids_string<S>(ids: &[Option<S>]) -> String
    where
        S: AsRef<str> + std::fmt::Display,
    {
        ids.iter()
            .fold(
                Rc::new(RefCell::new(String::from(""))),
                |ids_string, potential_id| match potential_id {
                    Some(id) => {
                        ids_string.replace_with(|old_ids_string| {
                            if old_ids_string.is_empty() {
                                id.to_string()
                            } else {
                                format!("{old_ids_string},{id}")
                            }
                        });
                        ids_string
                    }
                    None => ids_string,
                },
            )
            .take()
    }
}

#[cfg(test)]
mod baseline_test {
    use super::baseline::*;

    #[test]
    fn should_create_base_url_with_api_key() {
        let base_url = construct_base_url("123", None).unwrap();
        assert_eq!(base_url.query(), Some("apikey=123"));
    }

    #[test]
    fn should_create_base_url() {
        let base_url = construct_base_url("123", None).unwrap();
        assert_eq!(base_url.host_str(), Some("creativecommons.tankerkoenig.de"));
    }

    #[test]
    fn should_create_base_url_with_path() {
        let base_url = construct_base_url("123", Some("prices.php")).unwrap();
        assert_eq!(base_url.host_str(), Some("creativecommons.tankerkoenig.de"));
        assert_eq!(base_url.query(), Some("apikey=123"));
        assert_eq!(base_url.path(), "/prices.php")
    }
}

#[cfg(test)]
mod price_test {
    use super::price::*;

    #[test]
    fn should_create_string_of_ids() {
        // Construct string of ids out of array with potential ids
        let ids = vec![Some("123"), Some("456"), None, None, Some("789")];
        let ids_string = format_ids_string(&ids);
        assert_eq!(ids_string, "123,456,789");

        // Empty if array only None
        let ids: Vec<Option<String>> = vec![None, None];
        let ids_string = format_ids_string(&ids);
        assert_eq!(ids_string, "");

        // Should have first available id as first part of the id string
        let ids = vec![None, Some("123"), None, Some("456")];
        let ids_string = format_ids_string(&ids);
        assert_eq!(ids_string, "123,456");
    }
}
