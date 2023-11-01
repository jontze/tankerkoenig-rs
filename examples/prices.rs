use tankerkoenig::Tankerkoenig;

#[tokio::main]
async fn main() {
    // This is an example API Key. You can get your own at https://creativecommons.tankerkoenig.de/
    let client = Tankerkoenig::new("00000000-0000-0000-0000-000000000002").unwrap();

    let station_prices = client
        .price
        .fetch_one("474e5046-deaf-4f9b-9a32-9797b778f047")
        .await
        .unwrap();
    println!("{:#?}", station_prices);
}
