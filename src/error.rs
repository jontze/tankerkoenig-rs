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
    #[error("Failed to parse json response: '{body}'")]
    ResponseParsingError {
        /// Response body that could not be parsed
        body: String,
    },
    /// Something went wrong during header construction
    #[error("Failed to construct http header")]
    HeaderConstruction {
        /// Error source
        #[from]
        source: reqwest::header::InvalidHeaderValue,
    },
    /// Something went wrong during http client creation
    #[error("Failed to create http client")]
    ClientConstruction {
        /// Error source
        #[source]
        source: reqwest::Error,
    },
    /// Failed to parse the request url
    #[error("Failed to construct the url")]
    UrlConstruction,
    /// Informations for too many stations were requested
    #[error("The request is not possible as to many ids were given. Max: {max}")]
    TooManyStations {
        /// Maximum number of simultaneously requested stations exceeded
        max: usize,
    },
}
