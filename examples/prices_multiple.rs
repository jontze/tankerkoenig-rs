use tankerkoenig::{chunk_into_option_arrays, Tankerkoenig};

#[tokio::main]
async fn main() {
    // This is an example API Key. You can get your own at https://creativecommons.tankerkoenig.de/
    let client = Tankerkoenig::new("00000000-0000-0000-0000-000000000002").unwrap();

    // This represents all station ids that you want to fetch prices for.
    let station_ids = vec![
        "474e5046-deaf-4f9b-9a32-9797b778f047",
        "278130b1-e062-4a0f-80cc-19e486b4c024",
        "4429a7d9-fb2d-4c29-8cfe-2ca90323f9f8",
        "24a381e3-0d72-416d-bfd8-b2f65f6e5802",
        "60c0eefa-d2a8-4f5c-82cc-b5244ecae955",
        "446bdcf5-9f75-47fc-9cfa-2c3d6fda1c3b",
    ];

    // This will chunk the station ids vector into fixed sized arrays of 10 elements each.
    // This is necessary because the API only allows 10 station ids per request.
    let chunked_station_ids = chunk_into_option_arrays!(station_ids);

    for id_chunk in chunked_station_ids {
        let station_prices = client.price.fetch(&id_chunk).await.unwrap();
        println!("{:?}", station_prices);
    }
}
