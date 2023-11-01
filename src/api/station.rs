//! Station module that gives access to the [StationApi] struct
use crate::{
    api::{client::HttpClient, url::TankerUrl},
    error, models,
};
use std::sync::Arc;

/// Struct that holds the current reqwest client of the library and gives access to the station api of
/// the tankerkoenig API.
#[derive(Clone)]
pub struct StationApi {
    client: Arc<Box<dyn HttpClient>>,
    api_key: String,
}

impl std::fmt::Debug for StationApi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StationApi")
            .field("client", &self.client)
            .field("api_key", &"********")
            .finish()
    }
}

impl TankerUrl for StationApi {}

impl StationApi {
    pub(crate) fn new(client: Arc<Box<dyn HttpClient>>, api_key: String) -> Self {
        Self { client, api_key }
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
        let mut url = self.base_url(&self.api_key, Some("json/list.php"))?;
        url.query_pairs_mut()
            .append_pair("lat", &lat.to_string())
            .append_pair("lng", &lng.to_string())
            .append_pair("rad", &radius.to_string())
            .append_pair("type", "all");
        let res_body = self.client.get(&url).await?;
        serde_json::from_str::<models::station::AreaNearResponse>(&res_body)
            .map_err(|_| error::TankerkoenigError::ResponseParsingError { body: res_body })
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
        let mut url = self.base_url(&self.api_key, Some("json/list.php"))?;
        url.query_pairs_mut()
            .append_pair("lat", &lat.to_string())
            .append_pair("lng", &lng.to_string())
            .append_pair("rad", &radius.to_string())
            .append_pair("type", &fuel.to_string())
            .append_pair("sort", &sort.to_string());
        let res_body = self.client.get(&url).await?;
        serde_json::from_str::<models::station::AreaFuelResponse>(&res_body)
            .map_err(|_| error::TankerkoenigError::ResponseParsingError { body: res_body })
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
        let mut url = self.base_url(&self.api_key, Some("json/detail.php"))?;
        url.query_pairs_mut().append_pair("id", id);
        let res_body = self.client.get(&url).await?;
        serde_json::from_str::<models::station::DetailsResponse>(&res_body)
            .map_err(|_| error::TankerkoenigError::ResponseParsingError { body: res_body })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        api::client::MockHttpClient,
        models::{AreaFuelResponse, AreaNearResponse},
    };
    use futures::future::ready;
    use mockall::predicate::*;
    use std::pin::Pin;

    #[tokio::test]
    async fn should_fetch_near_stations() {
        let data_string = std::fs::read_to_string("./test/data/list_near.json").unwrap();
        let api_key = String::from("123");

        let mut mock_client = MockHttpClient::new();
        let closure_data_string = data_string.clone();
        mock_client
            .expect_get()
            .times(1)
            .with(eq(reqwest::Url::parse("https://creativecommons.tankerkoenig.de/json/list.php?apikey=123&lat=52.52&lng=13.4&rad=10&type=all").unwrap()))
            .returning(move |_| {
                let result = Ok(String::from(closure_data_string.clone()));
                Pin::from(Box::new(ready(result)))
            });

        let api = StationApi::new(Arc::new(Box::new(mock_client)), api_key);
        let res = api.fetch_near(52.52, 13.40, 10.0).await.unwrap();

        let station_response: AreaNearResponse =
            serde_json::from_str(&data_string.clone()).unwrap();

        assert_eq!(res, station_response);
    }

    #[tokio::test]
    async fn should_fetch_stations_by_fuel() {
        let data_string = std::fs::read_to_string("./test/data/list.json").unwrap();
        let api_key = String::from("123");

        let mut mock_client = MockHttpClient::new();
        let closure_data_string = data_string.clone();
        mock_client
            .expect_get()
            .times(1)
            .with(eq(reqwest::Url::parse("https://creativecommons.tankerkoenig.de/json/list.php?apikey=123&lat=52.52&lng=13.4&rad=10&type=diesel&sort=dist").unwrap()))
            .returning(move |_| {
                let result = Ok(String::from(closure_data_string.clone()));
                Pin::from(Box::new(ready(result)))
            });

        let api = StationApi::new(Arc::new(Box::new(mock_client)), api_key);
        let res = api
            .fetch_by_fuel(
                52.52,
                13.40,
                10.0,
                models::Fuel::Diesel,
                models::Sort::Distance,
            )
            .await
            .unwrap();

        let station_response: AreaFuelResponse =
            serde_json::from_str(&data_string.clone()).unwrap();

        assert_eq!(res, station_response);
    }

    #[tokio::test]
    async fn should_fetch_station_details() {
        let data_string = std::fs::read_to_string("./test/data/detail.json").unwrap();
        let api_key = String::from("123");

        let mut mock_client = MockHttpClient::new();
        let closure_data_string = data_string.clone();
        mock_client
            .expect_get()
            .times(1)
            .with(eq(reqwest::Url::parse(
                "https://creativecommons.tankerkoenig.de/json/detail.php?apikey=123&id=123",
            )
            .unwrap()))
            .returning(move |_| {
                let result = Ok(String::from(closure_data_string.clone()));
                Pin::from(Box::new(ready(result)))
            });

        let api = StationApi::new(Arc::new(Box::new(mock_client)), api_key);
        let res = api.fetch_details("123").await.unwrap();

        let station_response: models::station::DetailsResponse =
            serde_json::from_str(&data_string.clone()).unwrap();

        assert_eq!(res, station_response);
    }

    #[test]
    fn should_obfuscated_token_in_debug() {
        let api_key = String::from("123");
        let mock_client = MockHttpClient::new();

        let api = StationApi::new(Arc::new(Box::new(mock_client)), api_key);

        assert_eq!(
            format!("{:?}", api),
            "StationApi { client: MockHttpClient, api_key: \"********\" }"
        );
    }
}
