mod price;
mod station;

use super::utils;
use price::PriceApi;
use station::StationApi;

/// Settings struct that contains the api key the current package version and the package name.
/// The API key will be used as a query parameter in the requests and the package name and version
/// will be used by default as useragent if no other user agent is provided
#[derive(Debug, Clone)]
pub struct Settings {
    api_key: String,
    pub package_version: &'static str,
    pub package_name: &'static str,
}

/// The main struct of the crate giving access to the station and price api of tankerkoenig.
/// Crate a new instance of the struct with your api key as parameter and optional your custom
/// user agent. If you want to submit requests with a custom user-agent you can pass this as
/// second parameter.
///
/// ## Example
/// ```
/// use tankerkoenig::Tankerkoenig;
/// use tankerkoenig::models;
///
/// async fn request_station_Details() -> Result<models::station::DetailsResponse, tankerkoenig::Error> {
///   let tanker = Tankerkoenig::new("your-api-key", None);
///   let details = tanker.station.fetch_detail("id-of-the-fuel-station").await?;
///   Ok(details)
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Tankerkoenig {
    pub station: StationApi,
    pub price: PriceApi,
}

impl<'a> Tankerkoenig {
    /// Creates a new instance of the Tankerkoenig struct by passing your api key as first
    /// function parameter. Optional you can provide a custom user agent for the requests,
    /// otherwise just pass `None` as second parameter.
    ///
    /// ## Example
    /// ```
    /// use tankerkoenig::Tankerkoenig;
    /// use tankerkoenig::models;
    ///
    /// async fn request_station_Details() -> Result<models::station::DetailsResponse, tankerkoenig::Error> {
    ///   let tanker = Tankerkoenig::new("your-api-key", None);
    ///   let details = tanker.station.fetch_detail("id-of-the-fuel-station").await?;
    ///   Ok(details)
    /// }
    /// ```
    pub fn new(api_key: &'a str, user_agent: Option<&str>) -> Self {
        let settings = std::sync::Arc::new(Settings {
            package_version: env!("CARGO_PKG_VERSION"),
            package_name: env!("CARGO_PKG_NAME"),
            api_key: String::from(api_key),
        });
        let client = std::sync::Arc::new(
            utils::baseline::construct_client(user_agent, &settings)
                .expect("Client to be constructed"),
        );
        Self {
            station: StationApi::new(client.clone(), settings.clone()),
            price: PriceApi::new(client, settings),
        }
    }
}
