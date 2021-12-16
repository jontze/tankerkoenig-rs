# tankerkoenig-rs

A API wrapper for the [tankerkoenig-api](https://creativecommons.tankerkoenig.de/) written in rust.

Comes with ready deserialized structs.

## Installation

This crate is currently not release as it is still unter heavy development. However, if you still want to use it, you can install it by adding this to your `Cargo.toml`:

```toml

[dependencies]
tankerkoenig = { git = "https://github.com/jontze/tankerkoenig-rs" }

```

## Quickstart

```rust
use tankerkoenig::Tankerkoenig;
use tankerkoenig::models;

async fn request_station_details() -> Result<models::station::DetailsResponse, tankerkoenig::Error> {
    let tanker = Tankerkoenig::new("your-api-key", None);
    let details = tanker.station.fetch_detail("id-of-the-fuel-station").await?;
     Ok(details)
}
```
