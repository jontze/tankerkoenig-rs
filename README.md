# tankerkoenig-rs

[![GitHub license](https://img.shields.io/github/license/jontze/tankerkoenig-rs)](https://github.com/jontze/tankerkoenig-rs/blob/main/LICENSE)
![crates.io](https://img.shields.io/crates/v/tankerkoenig.svg)
![docs.rs](https://img.shields.io/docsrs/tankerkoenig)
[![Main](https://github.com/jontze/tankerkoenig-rs/actions/workflows/main.yml/badge.svg)](https://github.com/jontze/tankerkoenig-rs/actions/workflows/main.yml)

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
