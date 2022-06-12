# tankerkoenig-rs

API wrapper for the [tankerkoenig-api](https://creativecommons.tankerkoenig.de/) written in rust.

Gives you ready deserialized structs and a easy to use and strictly typed api.

## Installation

This crate is currently not released as it is still unter heavy development. However, if you still want to use it, you can install it by adding this to your `Cargo.toml`:

```toml

[dependencies]
tankerkoenig = { git = "https://github.com/jontze/tankerkoenig-rs" }

```

## Quickstart

```rust
use tankerkoenig::Tankerkoenig;
use tankerkoenig::models;

async fn request_station_details() -> Result<models::station::DetailsResponse, tankerkoenig::Error> {
    let tanker = Tankerkoenig::new("<your-api-key>")?;
    let details = tanker.station.fetch_details("id-of-a-fuel-station").await?;
     Ok(details)
}
```
