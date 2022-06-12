//! Module that contains the main [Tankerkoenig] struct that gives access to
//! the two child structs [PriceApi](price::PriceApi) and [StationApi](station::StationApi).

pub mod price;
pub mod station;

use crate::error::TankerkoenigError;
use crate::utils;
use price::PriceApi;
use station::StationApi;

/// Settings struct that contains the api key, the current package version and the package name.
/// The API key will be used as a query parameter in the requests and the package name and version
/// will be used by default as useragent if no other useragent is provided.
#[derive(Debug, Clone)]
pub struct Settings {
    api_key: String,
    /// Crate version
    pub package_version: &'static str,
    /// Crate name
    pub package_name: &'static str,
}

/// The main struct of the crate giving access to the station and price api of tankerkoenig.
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

impl<'a> Tankerkoenig {
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
    pub fn new(api_key: &'a str) -> Result<Self, TankerkoenigError> {
        let settings = std::sync::Arc::new(Settings {
            package_version: env!("CARGO_PKG_VERSION"),
            package_name: env!("CARGO_PKG_NAME"),
            api_key: String::from(api_key),
        });
        let client = utils::baseline::construct_client(None, &settings)?;
        Ok(Self {
            station: StationApi::new(client.clone(), settings.clone()),
            price: PriceApi::new(client, settings),
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
    pub fn new_with_useragent(
        api_key: &'a str,
        user_agent: &'a str,
    ) -> Result<Self, TankerkoenigError> {
        let settings = std::sync::Arc::new(Settings {
            package_version: env!("CARGO_PKG_VERSION"),
            package_name: env!("CARGO_PKG_NAME"),
            api_key: String::from(api_key),
        });
        let client = utils::baseline::construct_client(Some(user_agent), &settings)?;
        Ok(Self {
            station: StationApi::new(client.clone(), settings.clone()),
            price: PriceApi::new(client, settings),
        })
    }
}
