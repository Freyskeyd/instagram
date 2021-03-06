use async_trait::async_trait;

use headers::{HeaderMap, HeaderValue};
use regex::Regex;
use reqwest::header as headers;
use reqwest::Client as HttpClient;
use reqwest::Error as HttpError;
use reqwest::Response as HttpResponse;

use crate::web_api::{
    behaviour::{FetchUserFeed, FetchUserInfos},
    credentials::Credentials,
    domain::{UserFeed, UserInfos},
    error::ClientError,
    options::FetchUserFeedOptions,
    response::{
        ApiResponse, GraphQLResponse, LoginResponse, UserFeedResponse, UserInfosError,
        UserInfosResponse,
    },
};

mod authenticated;
mod builder;

pub use authenticated::AuthenticatedClient;

use builder::ClientBuilder;

/// Web api entrypoint Client
///
/// An Unauthenticated Web client to access the api
#[derive(PartialEq, Debug)]
pub struct Client {
    api_url: String,
    graphql_api_url: String,
    csrf_token: Option<String>,
    init_csrf_token: Option<String>,
    rollout_hash: Option<String>,
}

#[async_trait]
#[allow(clippy::empty_line_after_outer_attr)]
impl FetchUserFeed for Client {
    async fn fetch_user_feed(
        &self,
        user_id: &str,
        options: Option<FetchUserFeedOptions<'_, '_>>,
    ) -> Result<UserFeed, ClientError> {
        let client = HttpClient::new();

        let options = options.unwrap_or_default().set_user_id(user_id);

        let variables = serde_json::to_string(&options).unwrap();

        let query = vec![
            ("query_hash", "9dcf6e1a98bc7f6e92953d5a61027b98"),
            ("variables", &variables),
        ];

        client
            .get(&self.graphql_api_url)
            .query(&query)
            .send()
            .await?
            .json::<GraphQLResponse<UserFeedResponse>>()
            .await
            .map(|r| r.data.feed)
            .map_err(Into::into)
    }
}

#[async_trait]
impl FetchUserInfos for Client {
    async fn fetch_user_infos(&self, username: &str) -> Result<UserInfos, UserInfosError> {
        let endpoint = format!("{}/{}", self.api_url, username);
        let client = HttpClient::new();

        client
            .get(&endpoint)
            .query(&[("__a", "1")])
            .send()
            .await?
            .json::<ApiResponse<UserInfosResponse>>()
            .await
            .map(|r| r.graphql.user)
            .map_err(Into::into)
    }
}

impl std::default::Default for Client {
    fn default() -> Self {
        ClientBuilder::new().build()
    }
}

impl Client {
    /// Create a new client with default configuration.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use instagram::web_api::Client;
    ///
    /// let client = Client::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[doc(hidden)]
    #[must_use]
    pub fn new_with_url(url: &str, graphql_api_url: &str) -> Self {
        ClientBuilder::new()
            .set_api_url(url)
            .set_graphql_api_url(graphql_api_url)
            .build()
    }

    #[doc(hidden)]
    #[must_use]
    pub fn get_api_url(&self) -> &str {
        &self.api_url
    }

    /// Login with provided credentials
    ///
    /// # Examples
    ///
    /// ```rust
    /// use instagram::web_api::*;
    ///
    /// let client = Client::new();
    ///
    /// let creds: Credentials = Credentials {
    ///     username: "user",
    ///     password: "passw",
    /// };
    ///
    /// // let login_infos: LoginInfos = client.login(&creds).await?;
    /// ```
    ///
    /// # Errors
    ///
    /// Will return `Err` if the request fails on instagram api.
    /// Maybe due to an unknown error or a mistake in the credentials
    pub async fn login(
        mut self,
        credentials: &Credentials<'_>,
    ) -> Result<AuthenticatedClient, ClientError> {
        self.init_rollout_hash().await?;

        let mut headers = headers::HeaderMap::new();

        headers.insert(headers::ACCEPT, HeaderValue::from_static("*/*"));
        headers.insert(headers::ACCEPT_LANGUAGE, HeaderValue::from_static("en-US"));
        headers.insert(headers::CONNECTION, HeaderValue::from_static("close"));
        headers.insert(
            headers::CONTENT_TYPE,
            HeaderValue::from_static("application/x-www-form-urlencoded"),
        );

        let cookie = cookie::Cookie::build("ig_cb", "1")
            .domain("www.instagram.com")
            .path("/")
            .secure(false)
            .finish();

        if let Ok(cookie_header) = HeaderValue::from_str(&cookie.to_string()) {
            headers.insert(headers::COOKIE, cookie_header);
        }

        if let Some(ref csrf_token) = self.csrf_token {
            if let Ok(csrftoken) = HeaderValue::from_str(csrf_token) {
                headers.insert(headers::HeaderName::from_static("x-csrftoken"), csrftoken);
            }
        }

        headers.insert(
            headers::HeaderName::from_static("x-request-with"),
            HeaderValue::from_static("XMLHttpRequest"),
        );

        if let Some(ref rollout_hash) = self.rollout_hash {
            if let Ok(rollout_hash) = HeaderValue::from_str(rollout_hash) {
                headers.insert(
                    headers::HeaderName::from_static("x-instagram-ajax"),
                    rollout_hash,
                );
            }
        }

        headers.insert(
            headers::REFERER,
            HeaderValue::from_static("https://www.instagram.com"),
        );
        headers.insert(
            headers::HeaderName::from_static("authority"),
            HeaderValue::from_static("www.instagram.com"),
        );
        headers.insert(
            headers::ORIGIN,
            HeaderValue::from_static("https://www.instagram.com"),
        );

        let client = HttpClient::builder()
            .gzip(true)
            .cookie_store(true)
            .default_headers(headers)
            .build()
            .unwrap();

        let url = format!("{}/accounts/login/ajax/", self.api_url);
        let login_res: LoginResponse = client
            .post(&url)
            .form(&[
                ("username", credentials.username),
                ("password", credentials.password),
            ])
            .send()
            .await?
            .json()
            .await?;

        match login_res {
            LoginResponse::Success(login_infos) => {
                AuthenticatedClient::from_login_infos(self, &login_infos)
            }
            LoginResponse::TwoFactorNeeded(_) => Err(ClientError::UnableToPerform2FA),
        }
    }

    // """Make a GET request to get the first csrf token and rhx_gis"""
    async fn init(&mut self) -> Result<(), ClientError> {
        let response = self
            .make_request(&format!("{}/", self.api_url), None)
            .await?;
        let body = response.text_with_charset("utf-8").await?;

        lazy_static! {
            static ref RE_ROLLOUT_HASH: Regex =
                Regex::new(r#"rollout_hash":"(?P<rollout_hash>[A-Za-z0-9]+)"#).unwrap();
            static ref RE_CSRF_TOKEN: Regex =
                Regex::new(r#"csrf_token":"(?P<csrf_token>[A-Za-z0-9]+)"#).unwrap();
        }

        let _rollout_hash: Option<&str> = RE_ROLLOUT_HASH
            .captures(&body)
            .and_then(|cap| cap.name("rollout_hash").map(|v| v.as_str()));

        if self.csrf_token.is_none() {
            self.csrf_token = RE_CSRF_TOKEN
                .captures(&body)
                .and_then(|cap| cap.name("csrf_token").map(|v| v.as_str().to_string()));

            self.init_csrf_token = self.csrf_token.clone();
        }

        if self.csrf_token.is_none() {
            Err(ClientError::UnableToGetCsrfToken)
        } else {
            // println!("rollout: {:?}, csrf: {:?}", rollout_hash, self.csrf_token);

            Ok(())
        }
    }

    async fn init_rollout_hash(&mut self) -> Result<(), ClientError> {
        self.init().await
    }

    async fn make_request(
        &mut self,
        url: &str,
        credentials: Option<Credentials<'_>>,
    ) -> Result<HttpResponse, HttpError> {
        let mut headers = HeaderMap::new();

        headers.insert(headers::ACCEPT, HeaderValue::from_static("*/*"));
        headers.insert(headers::ACCEPT_LANGUAGE, HeaderValue::from_static("en-US"));
        headers.insert(headers::CONNECTION, HeaderValue::from_static("close"));
        headers.insert(
            headers::CONTENT_TYPE,
            HeaderValue::from_static("application/x-www-form-urlencoded"),
        );

        let client_builder = HttpClient::builder().gzip(true).cookie_store(true);

        if let Some(credentials) = credentials {
            let client = client_builder.default_headers(headers).build().unwrap();

            client
                .post(url)
                .form(&[
                    ("username", credentials.username),
                    ("password", credentials.password),
                ])
                .send()
                .await
        } else {
            let client = client_builder.default_headers(headers).build().unwrap();

            client.get(url).send().await
        }
    }
}
