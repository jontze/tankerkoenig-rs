#[derive(Error, Debug)]
pub enum TankerkoenigError {
    #[error("request to api failed")]
    RequestError {
        #[source]
        source: reqwest::Error,
    },
    #[error("Failed to parse json response")]
    ResponseParsingError {
        #[source]
        source: reqwest::Error,
    },
}
