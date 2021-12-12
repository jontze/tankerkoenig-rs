use super::Settings;
use crate::error;
use crate::models;
use crate::utils::{baseline::construct_base_url, price::format_ids_string};
use reqwest::Client;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct PriceApi {
    client: Arc<Client>,
    settings: Arc<Settings>,
}

impl PriceApi {
    pub fn new(client: Arc<Client>, settings: Arc<Settings>) -> Self {
        Self { client, settings }
    }

    pub async fn fetch(
        &self,
        ids: Vec<String>,
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
