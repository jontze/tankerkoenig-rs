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
}
