//! Station module that gives access to the [StationApi] struct

use crate::api::Settings;
use crate::error;
use crate::models;
use crate::utils::baseline::construct_base_url;
use reqwest::Client;
use std::sync::Arc;

/// Struct that holds the current reqwest client of the library and gives access to the station api of
/// the tankerkoenig API.
#[derive(Debug, Clone)]
pub struct StationApi {
    client: Client,
    options: Arc<Settings>,
}

impl StationApi {
    /// Creates a new instance of the [StationApi] struct that will give access to
    /// the station API of tankerkoenig.
    ///
    /// This struct is mainly used internally but can be used external as well. The
    /// recommended way is to use the [Tankerkoenig](crate::Tankerkoenig) struct and
    /// not to use the [StationApi] struct directly.
    ///
    /// However, if you you still want to do this, you need to pass
    /// an reqwest client and the Settings struct as parameter.
    pub fn new(client: Client, options: Arc<Settings>) -> Self {
        Self { client, options }
    }

    /// Fetch all stations near a given area with some basic informations
    ///
    /// ## Example
    /// ```
    /// use tankerkoenig::Tankerkoenig;
    /// use tankerkoenig::models;
    ///
    /// async fn request_near_stations() -> Result<models::AreaNearResponse, tankerkoenig::Error> {
    ///    let tanker = Tankerkoenig::new("your-api-key")?;
    ///    let latitude: f64 = 52.52;
    ///    let longitude: f64 = 13.40;
    ///    let radius: f64 = 10.0;
    ///    let near_stations_res = tanker.station.fetch_near(latitude, longitude, radius).await?;
    ///    Ok(near_stations_res)
    /// }
    /// ```
    pub async fn fetch_near(
        &self,
        lat: f64,
        lng: f64,
        radius: f64,
    ) -> Result<models::station::AreaNearResponse, error::TankerkoenigError> {
        let mut url = construct_base_url(&self.options.api_key, Some("json/list.php"))?;
        url.query_pairs_mut()
            .append_pair("lat", &lat.to_string())
            .append_pair("lng", &lng.to_string())
            .append_pair("rad", &radius.to_string())
            .append_pair("type", "all");
        let request = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|err| error::TankerkoenigError::RequestError { source: err })?
            .json::<models::station::AreaNearResponse>()
            .await
            .map_err(|err| error::TankerkoenigError::ResponseParsingError { source: err })?;
        Ok(request)
    }

    /// Fetch all stations in a radius around the given coordinates that sell a specific kind
    /// of [Fuel](models::Fuel) and [sort](models::Sort) them in a certain order.
    ///
    /// ## Example
    /// ```
    /// use tankerkoenig::Tankerkoenig;
    /// use tankerkoenig::models;
    ///
    /// async fn request_fuel_near() -> Result<models::AreaFuelResponse, tankerkoenig::Error> {
    ///    let tanker = Tankerkoenig::new("your-api-key")?;
    ///    let latitude: f64 = 52.52;
    ///    let longitude: f64 = 13.40;
    ///    let radius: f64 = 10.0;
    ///    let fuel = models::Fuel::Diesel;
    ///    let sorting = models::Sort::Distance;
    ///    let stations = tanker.station.fetch_by_fuel(latitude, longitude, radius, fuel, sorting).await?;
    ///    Ok(stations)
    /// }
    /// ```
    pub async fn fetch_by_fuel(
        &self,
        lat: f64,
        lng: f64,
        radius: f64,
        fuel: models::Fuel,
        sort: models::Sort,
    ) -> Result<models::station::AreaFuelResponse, error::TankerkoenigError> {
        let mut url = construct_base_url(&self.options.api_key, Some("json/list.php"))?;
        url.query_pairs_mut()
            .append_pair("lat", &lat.to_string())
            .append_pair("lng", &lng.to_string())
            .append_pair("rad", &radius.to_string())
            .append_pair("type", &fuel.to_string())
            .append_pair("sort", &sort.to_string());
        let request = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|err| error::TankerkoenigError::RequestError { source: err })?
            .json::<models::station::AreaFuelResponse>()
            .await
            .map_err(|err| error::TankerkoenigError::ResponseParsingError { source: err })?;
        Ok(request)
    }

    /// Fetch informations about a certain station by id.
    ///
    /// ## Example
    /// ```
    /// use tankerkoenig::Tankerkoenig;
    /// use tankerkoenig::models;
    ///
    /// async fn request_station() -> Result<models::DetailsResponse, tankerkoenig::Error> {
    ///    let tanker = Tankerkoenig::new("your-api-key")?;
    ///    let station_details = tanker.station.fetch_details("<station_id>").await?;
    ///    Ok(station_details)
    /// }
    /// ```
    pub async fn fetch_details<S: AsRef<str>>(
        &self,
        id: S,
    ) -> Result<models::station::DetailsResponse, error::TankerkoenigError> {
        let id = id.as_ref();
        let mut url = construct_base_url(&self.options.api_key, Some("json/detail.php"))?;
        url.query_pairs_mut().append_pair("id", id);
        let request = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|err| error::TankerkoenigError::RequestError { source: err })?
            .json::<models::station::DetailsResponse>()
            .await
            .map_err(|err| error::TankerkoenigError::ResponseParsingError { source: err })?;
        Ok(request)
    }
}
