//! This module contains rust structs that describe the response
//! of the tankerkoenig stations API.
//!
//! The json responses are parsed with serde to strongly typed
//! rust structs.

/// Response of the tankerkoenig API for a request of fuel
/// stations in a certain area
#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd)]
pub struct AreaNearResponse {
    /// Request status
    pub ok: bool,
    /// Data licence
    pub license: String,
    /// Data source
    pub data: String,
    /// Request status
    pub status: String,
    /// Vector of fuel stations in the area
    pub stations: Vec<NearStation>,
}

/// Information of a fuel station in the area returned by the
/// [AreaNearResponse].
#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
pub struct NearStation {
    /// Station id
    pub id: String,
    /// Name of the station
    pub name: String,
    /// Brand of the station (e.g. JET)
    pub brand: Option<String>,
    /// Street where the station is located
    pub street: String,
    /// Street number
    pub house_number: Option<String>,
    /// Local post code
    pub post_code: i64,
    /// Area of the station
    pub place: String,
    /// Latitude (geolocation)
    pub lat: f64,
    /// Longitude (geolocation)
    pub lng: f64,
    /// Distance from search point
    pub dist: f64,
    /// Diesel price
    pub diesel: Option<f64>,
    /// E5 price
    pub e5: Option<f64>,
    /// E10 price
    pub e10: Option<f64>,
    /// Open or Closed
    pub is_open: bool,
}

/// Response of the tankerkoenig API for a request for fuel in a
/// certain area
#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd)]
pub struct AreaFuelResponse {
    /// Request status
    pub ok: bool,
    /// Data licence
    pub license: String,
    /// Data source
    pub data: String,
    /// Request status
    pub status: String,
    /// List of stations in the area with the requested fuel
    pub stations: Vec<AreaStationFuel>,
}

/// Information about the fuel of a station in the area, returned
/// by the [AreaFuelResponse]
#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
pub struct AreaStationFuel {
    /// Station id
    pub id: String,
    /// Station name
    pub name: String,
    /// Brand of the station (e.g. JET)
    pub brand: Option<String>,
    /// Street where the station is located
    pub street: String,
    /// Street number
    pub house_number: Option<String>,
    /// Local post code
    pub post_code: i64,
    /// Area of the station
    pub place: String,
    /// Latitude (geolocation)
    pub lat: f64,
    /// Longitude (geolocation)
    pub lng: f64,
    /// Distance of the station
    pub dist: f64,
    /// Price of the requested fuel
    pub price: Option<f64>,
    /// Open or closed
    pub is_open: bool,
}

/// Response of the tankerkoenig API for a request of detailed
/// information for a specific fuel station
#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd)]
pub struct DetailsResponse {
    /// Request status
    pub ok: bool,
    /// Data licence
    pub license: String,
    /// Data source
    pub data: String,
    /// Request status
    pub status: String,
    /// Detailed fuel station information
    pub station: DetailStation,
}

/// Detailed information of a fuel station, returned by the [DetailsResponse]
#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
pub struct DetailStation {
    /// Station id
    pub id: String,
    /// Name of the station
    pub name: String,
    /// Brand of the station (e.g. JET)
    pub brand: String,
    /// Street where the station is located
    pub street: String,
    /// Street number
    pub house_number: Option<String>,
    /// Local post code
    pub post_code: i64,
    /// Area of the station
    pub place: String,
    /// Open 24 hours
    pub whole_day: bool,
    ///Currently open or closed
    pub is_open: bool,
    /// Diesel price
    pub diesel: Option<f64>,
    /// E5 price
    pub e5: Option<f64>,
    /// E10 price
    pub e10: Option<f64>,
    /// Latitude (geolocation)
    pub lat: f64,
    /// Longitude (geolocation)
    pub lng: f64,
    /// State of the station
    pub state: Option<String>,
    /// Additional information
    pub overrides: Vec<String>,
    /// Opening times of the station
    pub opening_times: Vec<OpeningTimes>,
}

/// Opening times for a fuel station
#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd)]
pub struct OpeningTimes {
    /// Information about the scope of start end end
    pub text: String,
    /// Opening time
    pub start: String,
    /// Closing time
    pub end: String,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn deserialize_area_station_response() {
        let data = std::fs::read_to_string("./test/data/list.json").unwrap();
        let station_response: AreaFuelResponse = serde_json::from_str(&data).unwrap();
        let station = station_response.stations.get(0).unwrap();
        assert_eq!(station_response.ok, true);
        assert_eq!(
            station_response.license,
            "CC BY 4.0 -  https://creativecommons.tankerkoenig.de"
        );
        assert_eq!(station_response.data, "MTS-K");
        assert_eq!(station_response.status, "ok");
        assert_eq!(station.id, "474e5046-deaf-4f9b-9a32-9797b778f047");
        assert_eq!(station.name, "TotalEnergies Berlin");
        assert_eq!(station.brand, Some(String::from("TotalEnergies")));
        assert_eq!(station.street, "Margarete-Sommer-Str.");
        assert_eq!(station.place, "Berlin");
        assert_eq!(station.lat, 52.530831);
        assert_eq!(station.lng, 13.440946);
        assert_eq!(station.dist, 0_f64);
        assert_eq!(station.price, Some(1.649));
        assert_eq!(station.is_open, true);
        assert_eq!(station.house_number, Some(String::from("2")));
        assert_eq!(station.post_code, 10407);
    }

    #[test]
    fn deserialize_area_stations_near_response() {
        let data = std::fs::read_to_string("./test/data/list_near.json").unwrap();
        let station_response: AreaNearResponse = serde_json::from_str(&data).unwrap();
        let station = station_response.stations.get(0).unwrap();
        assert_eq!(station_response.ok, true);
        assert_eq!(
            station_response.license,
            "CC BY 4.0 -  https://creativecommons.tankerkoenig.de"
        );
        assert_eq!(station_response.data, "MTS-K");
        assert_eq!(station_response.status, "ok");
        assert_eq!(station.id, "474e5046-deaf-4f9b-9a32-9797b778f047");
        assert_eq!(station.name, "TotalEnergies Berlin");
        assert_eq!(station.brand, Some(String::from("TotalEnergies")));
        assert_eq!(station.street, "Margarete-Sommer-Str.");
        assert_eq!(station.place, "Berlin");
        assert_eq!(station.lat, 52.530831);
        assert_eq!(station.lng, 13.440946);
        assert_eq!(station.dist, 0_f64);
        assert_eq!(station.diesel, Some(1.489));
        assert_eq!(station.e5, Some(1.649));
        assert_eq!(station.e10, Some(1.589));
        assert_eq!(station.is_open, true);
        assert_eq!(station.house_number, Some(String::from("2")));
        assert_eq!(station.post_code, 10407);
    }

    #[test]
    fn deserialize_detail_station_response() {
        let data = std::fs::read_to_string("./test/data/detail.json").unwrap();
        let station_detail_response: DetailsResponse = serde_json::from_str(&data).unwrap();
        let opening_times = station_detail_response
            .station
            .opening_times
            .get(0)
            .unwrap();
        assert_eq!(station_detail_response.ok, true);
        assert_eq!(
            station_detail_response.license,
            "CC BY 4.0 -  https://creativecommons.tankerkoenig.de"
        );
        assert_eq!(station_detail_response.data, "MTS-K");
        assert_eq!(station_detail_response.status, "ok");
        assert_eq!(
            station_detail_response.station.id,
            "24a381e3-0d72-416d-bfd8-b2f65f6e5802"
        );
        assert_eq!(station_detail_response.station.name, "Esso Tankstelle");
        assert_eq!(station_detail_response.station.brand, "ESSO");
        assert_eq!(station_detail_response.station.street, "HAUPTSTR. 7");
        assert_eq!(
            station_detail_response.station.house_number,
            Some(String::from(" "))
        );
        assert_eq!(station_detail_response.station.post_code, 84152);
        assert_eq!(station_detail_response.station.place, "MENGKOFEN");
        assert_eq!(station_detail_response.station.whole_day, false);
        assert_eq!(station_detail_response.station.is_open, false);
        assert_eq!(station_detail_response.station.e5, Some(1.379));
        assert_eq!(station_detail_response.station.e10, Some(1.359));
        assert_eq!(station_detail_response.station.diesel, Some(1.169));
        assert_eq!(station_detail_response.station.lat, 48.72210601);
        assert_eq!(station_detail_response.station.lng, 12.44438439);
        assert_eq!(station_detail_response.station.state, None);
        assert_eq!(
            station_detail_response.station.overrides.get(0).unwrap(),
            "13.04.2017, 15:00:00 - 13.11.2017, 15:00:00: geschlossen"
        );
        assert_eq!(opening_times.text, "Mo-Fr");
        assert_eq!(opening_times.start, "06:00:00");
        assert_eq!(opening_times.end, "22:30:00");
    }
}
