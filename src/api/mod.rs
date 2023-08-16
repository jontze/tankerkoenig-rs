//! Module that contains the main [Tankerkoenig] struct that gives access to
//! the two child structs [PriceApi](price::PriceApi) and [StationApi](station::StationApi).

mod client;
pub mod price;
pub mod station;
mod url;

use crate::error::TankerkoenigError;
use price::PriceApi;
use station::StationApi;
use std::sync::Arc;

/// The main struct of the crate giving access to the [`StationApi`] and [`PriceApi`] of tankerkoenig.
/// Create a new instance of the struct with your api key as parameter.
///
/// ## Example
/// ```
/// use tankerkoenig::Tankerkoenig;
/// use tankerkoenig::models;
///
/// async fn request_station_details() -> Result<models::station::DetailsResponse, tankerkoenig::Error> {
///   let tanker = Tankerkoenig::new("your-api-key")?;
///   let details = tanker.station.fetch_details("id-of-the-fuel-station").await?;
///   Ok(details)
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Tankerkoenig {
    /// Provide access to all station related api resources
    pub station: StationApi,
    /// Provide access to all price related api resources
    pub price: PriceApi,
}

impl Tankerkoenig {
    /// Creates a new instance of the Tankerkoenig struct by passing your api key as
    /// function parameter.
    ///
    /// ## Example
    /// ```
    /// use tankerkoenig::Tankerkoenig;
    /// use tankerkoenig::models;
    ///
    /// async fn request_station_details() -> Result<models::station::DetailsResponse, tankerkoenig::Error> {
    ///   let tanker = Tankerkoenig::new("your-api-key")?;
    ///   let details = tanker.station.fetch_details("id-of-the-fuel-station").await?;
    ///   Ok(details)
    /// }
    /// ```
    pub fn new<S>(api_key: S) -> Result<Self, TankerkoenigError>
    where
        S: AsRef<str>,
    {
        let api_key = api_key.as_ref().to_string();
        let client: Arc<Box<dyn client::HttpClient>> =
            Arc::new(Box::<client::HttpReqwestClientImpl>::default());
        Ok(Self {
            station: StationApi::new(client.clone(), api_key.clone()),
            price: PriceApi::new(client, api_key),
        })
    }

    /// Creates a new instance of the Tankerkoenig struct by passing your api key as first
    /// function parameter and your customer user agent as second parameter. Default user agent
    /// is the current crate name the version number.
    ///
    /// ## Example
    /// ```
    /// use tankerkoenig::Tankerkoenig;
    /// use tankerkoenig::models;
    ///
    /// async fn request_station_details() -> Result<models::station::DetailsResponse, tankerkoenig::Error> {
    ///   let user_agent = "Mozilla/5.0 (platform; rv:geckoversion) Gecko/geckotrail Firefox/firefoxversion";
    ///   let tanker = Tankerkoenig::new_with_useragent("your-api-key", user_agent)?;
    ///   let details = tanker.station.fetch_details("id-of-the-fuel-station").await?;
    ///   Ok(details)
    /// }
    /// ```
    pub fn new_with_useragent<S>(api_key: S, user_agent: S) -> Result<Self, TankerkoenigError>
    where
        S: AsRef<str>,
    {
        let api_key = api_key.as_ref().to_string();
        let client: Arc<Box<dyn client::HttpClient>> = Arc::new(Box::new(
            client::HttpReqwestClientImpl::new(user_agent.as_ref())?,
        ));
        Ok(Self {
            station: StationApi::new(client.clone(), api_key.clone()),
            price: PriceApi::new(client, api_key),
        })
    }
}
