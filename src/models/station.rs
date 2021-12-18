#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd)]
pub struct AreaNearResponse {
    pub ok: bool,
    pub license: String,
    pub data: String,
    pub status: String,
    pub stations: Vec<NearStation>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
pub struct NearStation {
    pub id: String,
    pub name: String,
    pub brand: Option<String>,
    pub street: String,
    pub place: String,
    pub lat: f64,
    pub lng: f64,
    pub dist: f64,
    pub diesel: Option<f64>,
    pub e5: Option<f64>,
    pub e10: Option<f64>,
    pub is_open: bool,
    pub house_number: Option<String>,
    pub post_code: i64,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd)]
pub struct AreaFuelResponse {
    pub ok: bool,
    pub license: String,
    pub data: String,
    pub status: String,
    pub stations: Vec<AreaStationFuel>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
pub struct AreaStationFuel {
    pub id: String,
    pub name: String,
    pub brand: Option<String>,
    pub street: String,
    pub place: String,
    pub lat: f64,
    pub lng: f64,
    pub dist: f64,
    pub price: Option<f64>,
    pub is_open: bool,
    pub house_number: Option<String>,
    pub post_code: i64,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd)]
pub struct DetailsResponse {
    pub ok: bool,
    pub license: String,
    pub data: String,
    pub status: String,
    pub station: DetailStation,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
pub struct DetailStation {
    pub id: String,
    pub name: String,
    pub brand: String,
    pub street: String,
    pub house_number: Option<String>,
    pub post_code: i64,
    pub place: String,
    pub whole_day: bool,
    pub is_open: bool,
    pub e5: f64,
    pub e10: f64,
    pub diesel: f64,
    pub lat: f64,
    pub lng: f64,
    pub state: Option<String>,
    pub overrides: Vec<String>,
    pub opening_times: Vec<OpeningTimes>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd)]
pub struct OpeningTimes {
    pub text: String,
    pub start: String,
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
        assert_eq!(station_detail_response.station.e5, 1.379);
        assert_eq!(station_detail_response.station.e10, 1.359);
        assert_eq!(station_detail_response.station.diesel, 1.169);
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
