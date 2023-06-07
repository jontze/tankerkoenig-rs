pub mod baseline {
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

pub mod price {
    use std::{fmt::Display, slice::ChunksExact};

    const SLICE_SIZE: usize = 10;

    pub fn format_ids_string(ids: impl IntoIterator<Item = impl AsRef<str> + Display>) -> String {
        let mut ids_string = String::from("");
        for (index, id) in ids.into_iter().enumerate() {
            ids_string = if index == 0 {
                id.to_string()
            } else {
                format!("{},{}", ids_string, id)
            };
        }
        ids_string
    }

    pub fn slice_station_ids<I>(station_ids: I) -> Vec<[Option<I::Item>; 10]>
    where
        I: Iterator,
        I::Item: AsRef<str> + Display,
    {
        let mut sliced_arrays: Vec<[Option<I::Item>; SLICE_SIZE]> = Vec::new();
        let ids = station_ids.into_iter().collect::<Vec<I::Item>>();
        let mut chunks: ChunksExact<I::Item> = ids.chunks_exact(SLICE_SIZE);
        while let Some(chunk) = chunks.next() {
            let mut array: [Option<&I::Item>; SLICE_SIZE] = Default::default();
            for (index, id) in chunk.iter().enumerate() {
                array[index] = Some(id);
            }
            sliced_arrays.push(array);
        }
        let remaining_entries = chunks.remainder();
        if !remaining_entries.is_empty() {
            let mut array: [Option<I::Item>; SLICE_SIZE] = Default::default();
            for (index, id) in remaining_entries.iter().enumerate() {
                let test = id.to_owned();
                array[index] = Some(test);
            }
            sliced_arrays.push(array);
        }
        sliced_arrays
    }
}

pub mod station {}

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
        let ids = vec!["123", "456"];
        let ids_string = format_ids_string(ids);
        assert_eq!(ids_string, "123,456");
    }

    #[test]
    fn should_transform_vector_into_vector_with_slices() {
        let ids = vec![
            "1".to_string(),
            "2".to_string(),
            "3".to_string(),
            "4".to_string(),
            "5".to_string(),
            "6".to_string(),
            "7".to_string(),
            "8".to_string(),
            "9".to_string(),
            "10".to_string(),
            "11".to_string(),
        ];
        let sliced = slice_station_ids(ids);
        assert_eq!(sliced.len(), 2);
        assert_eq!(sliced.get(0).unwrap().len(), 10);
        assert_eq!(sliced.get(1).unwrap().len(), 10);
    }

    #[test]
    fn should_transform_vector_into_vector_with_slices_with_remaining_entries() {
        let ids = vec![
            "1".to_string(),
            "2".to_string(),
            "3".to_string(),
            "4".to_string(),
            "5".to_string(),
            "6".to_string(),
            "7".to_string(),
            "8".to_string(),
            "9".to_string(),
            "10".to_string(),
            "11".to_string(),
        ];
        let sliced = slice_station_ids(ids);
        let remaining_values: Vec<&Option<String>> = sliced
            .get(1)
            .unwrap()
            .iter()
            .filter(|value| value.is_some())
            .collect();
        assert_eq!(remaining_values.len(), 1);
    }
}
