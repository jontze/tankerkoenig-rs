//! # tankerkoenig-rs
//!
//! The [Tankerkoenig][tankerkoenig] crate provides an easy to use wrapper over the
//! [tankerkoenig api][tankerkoenig_api].
//!
//! * Easy to use
//! * Async api calls with [reqwest][reqwest]
//! * Ready deserialized structs of the tankerkoenig responses
//! * Manages authentication for you, just pass your api token once
//!
//! ## Requirements
//! * Your own [tankerkoenig api key][tankerkoenig_api]
//! * Async runtime configured e.g. [tokio][tokio]
//!
//! ## Getting started
//! Install the latest version of this crate by adding the following line to your `Cargo.toml`
//! ```toml
//! [dependencies]
//! tankerkoenig = { git = "https://github.com/jontze/tankerkoenig-rs" }
//! ```
//! Import the crate and create a new instance of the [Tankerkoenig][tankerkoenig] struct.
//! ```
//! use tankerkoenig::Tankerkoenig;
//! use tankerkoenig::models;
//!
//! async fn run() -> Result<models::station::DetailsResponse, tankerkoenig::Error> {
//!   let tanker = Tankerkoenig::new("your-api-key")?;
//!   let details = tanker.station.fetch_details("id-of-the-fuel-station").await?;
//!   Ok(details)
//! }
//! ```
//!
//!
//! ## Examples
//!
//!
//! ## Optional Features
//!
//!
//! ## Troubleshooting
//! If you get a ResponseParsingError during usage of the crate this is very likely
//! due to an invalid input where the tankerkoenig api will throw an error or
//! due to some unexpected values that were returned by the api. E.g. sometimes the api
//! will return `false` instead of a number for certain fields or other fields were missing.
//!
//! In this case please check if your input is valid and if so create a bug report on the
//! crate [repository][tankerkoenig_rs_repo] and provide some information about your input.
//!
//! [tankerkoenig]: ./api/struct.Tankerkoenig.html
//! [tankerkoenig_rs_repo]: https://github.com/jontze/tankerkoenig-rs
//! [tankerkoenig_api]: https://creativecommons.tankerkoenig.de/
//! [reqwest]: https://crates.io/crates/reqwest
//! [tokio]: https://crates.io/crates/tokio

#![warn(missing_docs)]
#![deny(rustdoc::bare_urls)]
#![deny(rustdoc::invalid_codeblock_attributes)]
#![deny(rustdoc::broken_intra_doc_links)]

#[macro_use]
extern crate serde;
extern crate serde_json;
extern crate strum;
#[macro_use]
extern crate strum_macros;
extern crate reqwest;
#[macro_use]
extern crate thiserror;

pub mod api;
mod error;
pub mod models;
mod utils;

pub use api::Tankerkoenig;
pub use error::TankerkoenigError as Error;
