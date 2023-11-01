use tankerkoenig::Tankerkoenig;

#[tokio::main]
async fn main() {
    // This is an example API Key. You can get your own at https://creativecommons.tankerkoenig.de/
    let client = Tankerkoenig::new("00000000-0000-0000-0000-000000000002").unwrap();

    let lat = 52.521_f64;
    let lng = 13.438_f64;
    let rad = 10_f64;

    let stations_in_area = client.station.fetch_near(lat, lng, rad).await.unwrap();
    println!("{:#?}", stations_in_area);
}
