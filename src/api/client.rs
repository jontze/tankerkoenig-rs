use reqwest::{
    header::{HeaderMap, HeaderValue, CONTENT_TYPE},
    Client,
};

use crate::error::TankerkoenigError;

#[async_trait::async_trait]
#[cfg_attr(test, mockall::automock)]
pub(crate) trait HttpClient: std::fmt::Debug {
    async fn get(&self, url: &reqwest::Url) -> Result<String, TankerkoenigError>;
}

#[derive(Debug)]
pub(crate) struct HttpReqwestClientImpl {
    client: reqwest::Client,
}

impl HttpReqwestClientImpl {
    pub(crate) fn new(user_agent: &str) -> Result<Self, TankerkoenigError> {
        let mut headers = HeaderMap::new();
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_str("application/json").unwrap(),
        );
        let client = Client::builder()
            .user_agent(user_agent)
            .default_headers(headers)
            .build()
            .map_err(|err| TankerkoenigError::ClientConstruction { source: err })
            .unwrap();

        Ok(Self { client })
    }
}

impl Default for HttpReqwestClientImpl {
    fn default() -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_str("application/json").unwrap(),
        );
        let agent = format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        let client = Client::builder()
            .user_agent(agent)
            .default_headers(headers)
            .build()
            .unwrap();

        Self { client }
    }
}

#[async_trait::async_trait]
impl HttpClient for HttpReqwestClientImpl {
    async fn get(&self, url: &reqwest::Url) -> Result<String, TankerkoenigError> {
        self.client
            .get(url.clone())
            .send()
            .await
            .map_err(|err| TankerkoenigError::RequestError { source: err })?
            .text()
            .await
            .map_err(|err| TankerkoenigError::RequestError { source: err })
    }
}

#[cfg(test)]
mod test {
    use wiremock::{matchers::path, MockServer};

    use super::*;

    #[tokio::test]
    async fn test_get() {
        // Start Http Server in Background
        let mock_server = MockServer::start().await;

        // Define behaviour
        wiremock::Mock::given(wiremock::matchers::method("GET"))
            .and(path("/hello"))
            .respond_with(wiremock::ResponseTemplate::new(200).set_body_string("Hello World!"))
            .mount(&mock_server)
            .await;

        // Create Client
        let client = HttpReqwestClientImpl::default();
        let url = reqwest::Url::parse(&format!("{}/hello", &mock_server.uri())).unwrap();

        // Send Request
        let res = client.get(&url).await.unwrap();

        // Assert Response
        assert_eq!(res, "Hello World!")
    }

    #[tokio::test]
    async fn test_get_with_useragent() {
        // Start Http Server in Background
        let mock_server = MockServer::start().await;

        // Define behaviour
        wiremock::Mock::given(wiremock::matchers::method("GET"))
            .and(path("/hello"))
            .respond_with(wiremock::ResponseTemplate::new(200).set_body_string("Hello World!"))
            .mount(&mock_server)
            .await;

        // Create Client
        let client = HttpReqwestClientImpl::new("my-user-agent").unwrap();
        let url = reqwest::Url::parse(&format!("{}/hello", &mock_server.uri())).unwrap();

        // Send Request
        let res = client.get(&url).await.unwrap();

        // Assert Response
        assert_eq!(res, "Hello World!")
    }
}
