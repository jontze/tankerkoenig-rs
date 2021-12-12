use super::Settings;
use crate::error;
use crate::models;
use crate::utils::baseline::construct_base_url;
use reqwest::Client;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct StationApi {
    client: Arc<Client>,
    options: Arc<Settings>,
}

impl StationApi {
    pub fn new(client: Arc<Client>, options: Arc<Settings>) -> Self {
        Self { client, options }
    }

    pub async fn fetch_near(
        &self,
        lat: f64,
        lng: f64,
        radius: f64,
    ) -> Result<models::station::AreaNearResponse, error::TankerkoenigError> {
        let mut url = construct_base_url(&self.options.api_key, Some("json/list.php"));
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

    pub async fn fetch_by_fuel(
        &self,
        lat: f64,
        lng: f64,
        radius: f64,
        fuel: models::Fuel,
        sort: models::Sort,
    ) -> Result<models::station::AreaFuelResponse, error::TankerkoenigError> {
        let mut url = construct_base_url(&self.options.api_key, Some("json/list.php"));
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

    pub async fn fetch_detail(
        &self,
        id: &str,
    ) -> Result<models::station::DetailsResponse, error::TankerkoenigError> {
        let mut url = construct_base_url(&self.options.api_key, Some("json/detail.php"));
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
