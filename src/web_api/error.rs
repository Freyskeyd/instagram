#[derive(PartialEq, Debug)]
pub enum ClientError {
    UnableToPerform2FA,
    UnableToGetCsrfToken,
    HttpRequest,
}

impl From<reqwest::Error> for ClientError {
    fn from(e: reqwest::Error) -> Self {
        println!("{:?}", e);
        Self::HttpRequest
    }
}
