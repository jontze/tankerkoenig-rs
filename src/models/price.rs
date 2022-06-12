//! This module contains rust structs that describe the response of the tankerkoenig
//! prices API.
//!
//! The json responses are parsed with serde to strongly typed rust structs.

use serde::{Deserialize, Deserializer};
use std::collections::HashMap;

/// Response of the tankerkoenig api mapped to a rust struct with serde.
/// The struct holds information about fuel prices for a collection of stations.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct PriceResponse {
    /// Request status
    pub ok: bool,
    /// Data licence
    pub license: String,
    /// Data licence
    pub data: String,
    /// Fuel prices of requested stations
    pub prices: HashMap<String, StationPrices>,
}

/// Fuel prices of a station. If one field is `Noone` the station doesn't offer the
/// fuel or it is currently not possibleto fetch the price.
#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd)]
pub struct StationPrices {
    /// Station open or closed
    pub status: String,
    /// Price for E5
    #[serde(deserialize_with = "de_from_bool_or_number", default = "default_fuel")]
    pub e5: Option<f64>,
    /// Price for E10
    #[serde(deserialize_with = "de_from_bool_or_number", default = "default_fuel")]
    pub e10: Option<f64>,
    /// Price for diesel
    #[serde(deserialize_with = "de_from_bool_or_number", default = "default_fuel")]
    pub diesel: Option<f64>,
}

/// This function should just return always null as fallback.
/// This is required as the custom deserialization can't handle missing values.
fn default_fuel() -> Option<f64> {
    if 1 < 0 {
        Some(1_f64)
    } else {
        None
    }
}

fn de_from_bool_or_number<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    let parsed_value = Option::<f64>::deserialize(deserializer);
    match parsed_value {
        Ok(value) => {
            Ok(value)
        }
        Err(_) => {            
            Ok(None)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn deserialize_station_price_response() {
        let data = std::fs::read_to_string("./test/data/prices.json").unwrap();
        let prices_response: PriceResponse = serde_json::from_str(&data).unwrap();
        assert_eq!(prices_response.ok, true);
        assert_eq!(
            prices_response.license,
            "CC BY 4.0 -  https://creativecommons.tankerkoenig.de"
        );
        assert_eq!(prices_response.data, "MTS-K");
        assert_eq!(
            prices_response
                .prices
                .get("60c0eefa-d2a8-4f5c-82cc-b5244ecae955")
                .unwrap()
                .status,
            "open"
        );
        assert_eq!(
            prices_response
                .prices
                .get("60c0eefa-d2a8-4f5c-82cc-b5244ecae955")
                .unwrap()
                .e5,
            None
        );
        assert_eq!(
            prices_response
                .prices
                .get("60c0eefa-d2a8-4f5c-82cc-b5244ecae955")
                .unwrap()
                .e10,
            None
        );
        assert_eq!(
            prices_response
                .prices
                .get("60c0eefa-d2a8-4f5c-82cc-b5244ecae955")
                .unwrap()
                .diesel,
            Some(1.189)
        );
        assert_eq!(
            prices_response
                .prices
                .get("446bdcf5-9f75-47fc-9cfa-2c3d6fda1c3b")
                .unwrap()
                .status,
            "closed"
        );
        assert_eq!(
            prices_response
                .prices
                .get("446bdcf5-9f75-47fc-9cfa-2c3d6fda1c3b")
                .unwrap()
                .e5,
            None
        );
        assert_eq!(
            prices_response
                .prices
                .get("446bdcf5-9f75-47fc-9cfa-2c3d6fda1c3b")
                .unwrap()
                .e10,
            None
        );
        assert_eq!(
            prices_response
                .prices
                .get("446bdcf5-9f75-47fc-9cfa-2c3d6fda1c3b")
                .unwrap()
                .diesel,
            None
        );
        assert_eq!(
            prices_response
                .prices
                .get("4429a7d9-fb2d-4c29-8cfe-2ca90323f9f8")
                .unwrap()
                .status,
            "open"
        );
        assert_eq!(
            prices_response
                .prices
                .get("4429a7d9-fb2d-4c29-8cfe-2ca90323f9f8")
                .unwrap()
                .e5,
            Some(1.409)
        );
        assert_eq!(
            prices_response
                .prices
                .get("4429a7d9-fb2d-4c29-8cfe-2ca90323f9f8")
                .unwrap()
                .e10,
            Some(1.389)
        );
        assert_eq!(
            prices_response
                .prices
                .get("4429a7d9-fb2d-4c29-8cfe-2ca90323f9f8")
                .unwrap()
                .diesel,
            Some(1.129)
        );
        assert_eq!(
            prices_response
                .prices
                .get("44444444-4444-4444-4444-444444444444")
                .unwrap()
                .status,
            "no prices"
        );
        assert_eq!(
            prices_response
                .prices
                .get("44444444-4444-4444-4444-444444444444")
                .unwrap()
                .e5,
            None
        );
        assert_eq!(
            prices_response
                .prices
                .get("44444444-4444-4444-4444-444444444444")
                .unwrap()
                .e10,
            None
        );
        assert_eq!(
            prices_response
                .prices
                .get("44444444-4444-4444-4444-444444444444")
                .unwrap()
                .diesel,
            None
        );
    }
}
