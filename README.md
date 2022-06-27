# tankerkoenig-rs

[![GitHub license](https://img.shields.io/github/license/jontze/tankerkoenig-rs)](https://github.com/jontze/tankerkoenig-rs/blob/main/LICENSE)
![crates.io](https://img.shields.io/crates/v/tankerkoenig.svg)
![docs.rs](https://img.shields.io/docsrs/tankerkoenig)
[![Main](https://github.com/jontze/tankerkoenig-rs/actions/workflows/main.yml/badge.svg)](https://github.com/jontze/tankerkoenig-rs/actions/workflows/main.yml)

API wrapper for the [tankerkoenig-api](https://creativecommons.tankerkoenig.de/) written in rust.

The API of [tankerkoenig](https://creativecommons.tankerkoenig.de/) gives you realtime fuel prices for germany with Creative Commons License. This rust wrapper provides you ready deserialized structs and an easy to use and strictly typed api.

## Installation

This crate is under development. Especially the response parsing needs some more testing. However, if you still want to use it, you can install it by adding this to your `Cargo.toml`:

```toml
[dependencies]
tankerkoenig = "0.1.0-rc.2"
# If you want to use the latest unreleased version:
tankerkoenig = { git = "https://github.com/jontze/tankerkoenig-rs" }

```

## Requirements

1. Api Token for the [tankerkoenig-api](https://creativecommons.tankerkoenig.de/)
2. Async runtime like [tokio](https://crates.io/crates/tokio)

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
