//! Price module that gives access to the [PriceApi] struct

use super::Settings;
use crate::error;
use crate::models;
use crate::utils::{baseline::construct_base_url, price::format_ids_string};
use reqwest::Client;
use std::fmt::Display;
use std::sync::Arc;

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
#[derive(Debug, Clone)]
pub struct PriceApi {
    client: Client,
    settings: Arc<Settings>,
}

impl PriceApi {
    /// Creates a new instance of the [PriceApi] struct that will give access to the price API
    /// of tankerkoenig.
    ///
    /// This struct is mainly used internally but can be used external as well. The
    /// recommended way is to use the [Tankerkoenig](crate::Tankerkoenig) struct and
    /// not to use the [PriceApi] struct directly.
    ///
    /// However, if you you still want to do this, you need to pass
    /// an reqwest client and the Settings struct as parameter.
    pub fn new(client: Client, settings: Arc<Settings>) -> Self {
        Self { client, settings }
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
        let mut url = construct_base_url(&self.settings.api_key, Some("json/prices.php"))?;
        url.query_pairs_mut()
            .append_pair("ids", &format_ids_string(ids));
        let res_body = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|err| error::TankerkoenigError::RequestError { source: err })?
            .text()
            .await
            .map_err(|err| error::TankerkoenigError::RequestError { source: err })?;
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
        let mut url = construct_base_url(&self.settings.api_key, Some("json/prices.php"))?;
        url.query_pairs_mut().append_pair("ids", id.as_ref());
        let res_body = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|err| error::TankerkoenigError::RequestError { source: err })?
            .text()
            .await
            .map_err(|err| error::TankerkoenigError::RequestError { source: err })?;
        serde_json::from_str::<models::price::PriceResponse>(&res_body)
            .map_err(|_| error::TankerkoenigError::ResponseParsingError { body: res_body })
    }
}
