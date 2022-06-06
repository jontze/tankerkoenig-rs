//! Price module that gives access to the [PriceApi] struct

use super::Settings;
use crate::error;
use crate::models;
use crate::utils::{baseline::construct_base_url, price::format_ids_string};
use reqwest::Client;
use std::sync::Arc;

/// Struct that holds the current reqwest client of the library and gives access to the price api of
/// the tankerkoenig API.
///
/// ## Example
/// ```
/// use tankerkoenig::Tankerkoenig;
/// use tankerkoenig::models;
///
/// async fn request_station_prices() -> Result<models::PriceResponse, tankerkoenig::Error> {
///    let tanker = Tankerkoenig::new("your-api-key");
///    let prices = tanker.price.fetch(vec!["station-id-1", "station-id-1"]).await?;
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

    /// Fetch the prices of all fuel types of the given station ids.
    ///
    /// ## Example
    /// ```
    /// use tankerkoenig::Tankerkoenig;
    /// use tankerkoenig::models;
    ///
    /// async fn request_station_prices() -> Result<models::PriceResponse, tankerkoenig::Error> {
    ///    let tanker = Tankerkoenig::new("your-api-key");
    ///    let prices = tanker.price.fetch(vec!["station-id-1", "station-id-1"]).await?;
    ///    Ok(prices)
    /// }
    /// ```
    pub async fn fetch(
        &self,
        ids: Vec<&str>,
    ) -> Result<models::price::PriceResponse, error::TankerkoenigError> {
        let mut url = construct_base_url(&self.settings.api_key, Some("json/prices.php"));
        url.query_pairs_mut()
            .append_pair("ids", &format_ids_string(ids));
        let request = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|err| error::TankerkoenigError::RequestError { source: err })?
            .json::<models::price::PriceResponse>()
            .await
            .map_err(|err| error::TankerkoenigError::ResponseParsingError { source: err })?;
        Ok(request)
    }
}
