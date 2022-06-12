#[derive(Error, Debug)]

/// Contains all possible errors of the crate
pub enum TankerkoenigError {
    /// Something went wrong during fetching of
    /// the tankerkoenig api
    #[error("request to api failed")]
    RequestError {
        /// Error source
        #[source]
        source: reqwest::Error,
    },
    /// Something went wrong during the parsing
    /// of the tankerkoenig api response.
    #[error("Failed to parse json response")]
    ResponseParsingError {
        /// Error source
        #[source]
        source: reqwest::Error,
    },
    /// Somehing went wrong during header construction
    #[error("Failed to construct http header")]
    HeaderConstruction {
        /// Error source
        #[from]
        source: reqwest::header::InvalidHeaderValue,
    },
    /// Somthing went wrong during http client creation
    #[error("Failed to create http client")]
    ClientConstruction {
        /// Error source
        #[source]
        source: reqwest::Error,
    },
    /// Failed to parse the request url
    #[error("Failed to construct the url")]
    UrlConstruction,
}
