//! Price module that gives access to the [PriceApi] struct

use crate::{
    api::{client::HttpClient, url::TankerUrl},
    error, models,
    utils::price::format_ids_string,
};
use std::{fmt::Display, sync::Arc};

const MAX_REQUEST_STATION_IDS: usize = 10;

/// Struct that holds the current reqwest client of the library and gives access to the price api of
/// the tankerkoenig API.
///
/// ## Example
/// ```
/// use tankerkoenig::Tankerkoenig;
/// use tankerkoenig::models;
///
/// async fn request_station_prices() -> Result<models::PriceResponse, tankerkoenig::Error> {
///    let tanker = Tankerkoenig::new("your-api-key")?;
///    let prices = tanker.price.fetch(&[Some("station-id-1"), Some("station-id-2"), Some("station-id-3"), Some("station-id-4"), Some("station-id-5"), None, None, None, None, None]).await?;
///    Ok(prices)
/// }
/// ```
#[derive(Clone)]
pub struct PriceApi {
    client: Arc<Box<dyn HttpClient>>,
    api_key: String,
}

impl std::fmt::Debug for PriceApi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PriceApi")
            .field("client", &self.client)
            .field("api_key", &"********")
            .finish()
    }
}

impl TankerUrl for PriceApi {}

impl PriceApi {
    pub(crate) fn new(client: Arc<Box<dyn HttpClient>>, api_key: String) -> Self {
        Self { client, api_key }
    }

    /// Fetch the prices of all fuel types of the given station ids (up to 10 at once).
    ///
    /// ## Further Explanation
    /// You can only fetch 10 stations at once. If you want to fetch more than 10 stations,
    /// you have to call this function multiple times. This is due to a limitation of the
    /// [tankerkoenig API](https://creativecommons.tankerkoenig.de/).
    /// Read more about that on their [website](https://creativecommons.tankerkoenig.de/)
    ///
    /// ## Example
    /// ```
    /// use tankerkoenig::Tankerkoenig;
    /// use tankerkoenig::models;
    ///
    /// async fn request_station_prices() -> Result<models::PriceResponse, tankerkoenig::Error> {
    ///    let tanker = Tankerkoenig::new("your-api-key")?;
    ///    let prices = tanker.price.fetch(&[Some("station-id-1"), Some("station-id-2"), Some("station-id-3"), Some("station-id-4"), Some("station-id-5"), None, None, None, None, None]).await?;
    ///    Ok(prices)
    /// }
    /// ```
    pub async fn fetch<S>(
        &self,
        ids: &[Option<S>; MAX_REQUEST_STATION_IDS],
    ) -> Result<models::price::PriceResponse, error::TankerkoenigError>
    where
        S: AsRef<str> + Display,
    {
        let mut url = self.base_url(&self.api_key, Some("json/prices.php"))?;
        url.query_pairs_mut()
            .append_pair("ids", &format_ids_string(ids));
        let res_body = self.client.get(&url).await?;
        serde_json::from_str::<models::price::PriceResponse>(&res_body)
            .map_err(|_| error::TankerkoenigError::ResponseParsingError { body: res_body })
    }

    /// Fetch the prices of a single station
    ///
    /// ## Example
    /// ```
    /// use tankerkoenig::Tankerkoenig;
    /// use tankerkoenig::models;
    ///
    /// async fn request_single_station_prices() -> Result<models::PriceResponse, tankerkoenig::Error> {
    ///    let tanker = Tankerkoenig::new("your-api-key")?;
    ///    let station_prices = tanker.price.fetch_one("station-id").await?;
    ///    Ok(station_prices)
    /// }
    /// ```
    pub async fn fetch_one<S>(
        &self,
        id: S,
    ) -> Result<models::price::PriceResponse, error::TankerkoenigError>
    where
        S: AsRef<str> + Display,
    {
        let mut url = self.base_url(&self.api_key, Some("json/prices.php"))?;
        url.query_pairs_mut().append_pair("ids", id.as_ref());
        let res_body = self.client.get(&url).await?;
        serde_json::from_str::<models::price::PriceResponse>(&res_body)
            .map_err(|_| error::TankerkoenigError::ResponseParsingError { body: res_body })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::api::client::MockHttpClient;
    use futures::future::ready;
    use mockall::predicate::*;
    use std::pin::Pin;

    #[tokio::test]
    async fn should_fetch_station_price() {
        let data_string = std::fs::read_to_string("./test/data/prices.json").unwrap();
        let api_key = String::from("123");

        let mut mock_client = MockHttpClient::new();
        let closure_data_string = data_string.clone();
        mock_client
            .expect_get()
            .times(1)
            .with(eq(reqwest::Url::parse(
                "https://creativecommons.tankerkoenig.de/json/prices.php?apikey=123&ids=456",
            )
            .unwrap()))
            .returning(move |_| {
                let result = Ok(String::from(closure_data_string.clone()));
                Pin::from(Box::new(ready(result)))
            });

        let api = PriceApi::new(Arc::new(Box::new(mock_client)), api_key);
        let res = api.fetch_one("456").await.unwrap();

        let station_prices: models::price::PriceResponse =
            serde_json::from_str(&data_string).unwrap();

        assert_eq!(res, station_prices);
    }

    #[tokio::test]
    async fn should_fetch_station_prices() {
        let data_string = std::fs::read_to_string("./test/data/prices.json").unwrap();
        let api_key = String::from("123");

        let mut mock_client = MockHttpClient::new();
        let closure_data_string = data_string.clone();
        mock_client
            .expect_get()
            .times(1)
            .with(eq(reqwest::Url::parse(
                "https://creativecommons.tankerkoenig.de/json/prices.php?apikey=123&ids=456%2C789",
            )
            .unwrap()))
            .returning(move |_| {
                let result = Ok(String::from(closure_data_string.clone()));
                Pin::from(Box::new(ready(result)))
            });

        let api = PriceApi::new(Arc::new(Box::new(mock_client)), api_key);
        let res = api
            .fetch(&[
                Some("456"),
                Some("789"),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ])
            .await
            .unwrap();

        let station_prices: models::price::PriceResponse =
            serde_json::from_str(&data_string).unwrap();

        assert_eq!(res, station_prices);
    }

    #[test]
    fn should_obfuscated_token_in_debug() {
        let api_key = String::from("123");
        let mock_client = MockHttpClient::new();

        let api = PriceApi::new(Arc::new(Box::new(mock_client)), api_key);

        assert_eq!(
            format!("{:?}", api),
            "PriceApi { client: MockHttpClient, api_key: \"********\" }"
        );
    }
}
