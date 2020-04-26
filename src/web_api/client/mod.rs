use headers::{HeaderMap, HeaderValue};
use regex::Regex;
use reqwest::header as headers;
use reqwest::Client as HttpClient;
use reqwest::Error as HttpError;
use reqwest::Response as HttpResponse;

use crate::web_api::{
    credentials::Credentials,
    domain::UserInfos,
    error::ClientError,
    response::{ApiResponse, LoginResponse, UserInfosError, UserInfosResponse},
};

mod authenticated;

pub use authenticated::AuthenticatedClient;

struct ClientBuilder<'a> {
    url: &'a str,
}

impl<'a> ClientBuilder<'a> {
    const fn new() -> Self {
        Self {
            url: "https://www.instagram.com",
        }
    }

    const fn set_api_url(mut self, url: &'a str) -> Self {
        self.url = url;

        self
    }

    fn build(self) -> Client {
        Client {
            api_url: self.url.to_string(),
            csrf_token: None,
            init_csrf_token: None,
            rollout_hash: None,
        }
    }
}

/// Web api entrypoint Client
///
/// An Unauthenticated Web client to access the api
#[derive(PartialEq, Debug)]
pub struct Client {
    api_url: String,
    csrf_token: Option<String>,
    init_csrf_token: Option<String>,
    rollout_hash: Option<String>,
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
    pub fn new() -> Self {
        Self::default()
    }

    #[doc(hidden)]
    pub fn new_with_url(url: &str) -> Self {
        ClientBuilder::new().set_api_url(url).build()
    }

    #[doc(hidden)]
    pub fn get_api_url(&self) -> &str {
        &self.api_url
    }

    /// Fetch user's informations
    ///
    /// # Examples
    ///
    /// ```rust
    /// use instagram::web_api::*;
    ///
    /// let client = Client::new();
    ///
    /// //let some_user_info: UserInfos = client.user_infos("SomeUser").await?;
    /// ```
    pub async fn user_infos(&self, username: &str) -> Result<UserInfos, UserInfosError> {
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
    ///     password: "passw"
    /// };
    ///
    /// // let login_infos: LoginInfos = client.login(&creds).await?;
    /// ```
    pub async fn login(
        &mut self,
        credentials: &Credentials<'_>,
    ) -> Result<AuthenticatedClient, ClientError> {
        self._init_rollout_hash().await?;

        let mut headers = headers::HeaderMap::new();

        let _cookie = cookie::Cookie::build("ig_cb", "1")
            .domain("www.instagram.com")
            .path("/")
            .secure(false)
            .finish();

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
            ._make_request(&format!("{}/", self.api_url), None)
            .await?;
        let body = response.text_with_charset("utf-8").await?;

        lazy_static! {
            static ref RE_ROLLOUT_HASH: Regex =
                Regex::new(r#"rollout_hash":"(?P<rollout_hash>[A-Za-z0-9]+)"#).unwrap();
            static ref RE_CSRF_TOKEN: Regex =
                Regex::new(r#"csrf_token":"(?P<csrf_token>[A-Za-z0-9]+)"#).unwrap();
        }

        let rollout_hash: Option<&str> = RE_ROLLOUT_HASH
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
            println!("rollout: {:?}, csrf: {:?}", rollout_hash, self.csrf_token);

            Ok(())
        }
    }

    async fn _init_rollout_hash(&mut self) -> Result<(), ClientError> {
        self.init().await
    }

    async fn _make_request(
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
