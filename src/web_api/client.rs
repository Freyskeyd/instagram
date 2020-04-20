use super::domain::UserInfo;
use super::response::{ ApiResponse, UserInfoResponse, UserInfoError };

pub struct Client {
    api_url: String,
    pub version: i8
}

impl Client {
    pub fn new() -> Client {
        Client {
            api_url: "https://www.instagram.com".into(),
            version: 0
        }
    }

    pub fn new_with_url(url: &str) -> Client {
        Client {
            api_url: url.into(),
            version: 0
        }
    }

    pub fn authenticated_user_id(&self) -> String {
        String::new()
    }

    pub fn authenticated_user_name(&self) -> String {
        String::new()
    }

    pub fn is_authenticated(&self) -> bool {
        false
    }

    pub async fn user_info(&self, username: &str) -> Result<UserInfo, UserInfoError> {
        let endpoint = format!("{}/{}", self.api_url, username);
        let client = reqwest::Client::new();

        client.get(&endpoint).query(&[( "__a", "1" )])
            .send()
            .await?
            .json::<ApiResponse<UserInfoResponse>>()
            .await
            .map(|r| r.graphql.user)
            .map_err(Into::into)
    }
}
