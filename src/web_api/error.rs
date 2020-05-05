#[derive(PartialEq, Debug)]
pub enum ClientError {
    UnableToPerform2FA,
    UnableToGetCsrfToken,
    HttpRequest,
}

impl From<reqwest::Error> for ClientError {
    fn from(_e: reqwest::Error) -> Self {
        Self::HttpRequest
    }
}
