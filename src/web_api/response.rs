use super::domain::UserInfo;
use serde::Deserialize;

impl std::convert::From<reqwest::Error> for UserInfoError {
    fn from(_error: reqwest::Error) -> Self {
        Self::Other
    }
}

#[derive(Debug)]
pub enum UserInfoError {
    NotFound,
    Other
}

#[derive(Deserialize)]
pub struct ApiResponse<T> {
    pub graphql: T
}

#[derive(Deserialize)]
pub struct UserInfoResponse {
    pub user: UserInfo
}

