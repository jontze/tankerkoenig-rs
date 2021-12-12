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
