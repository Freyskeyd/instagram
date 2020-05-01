use super::Client;
use crate::web_api::domain::LoginInfos;
use crate::web_api::error::ClientError as Error;
use async_trait::async_trait;

use crate::web_api::{behaviour::FetchUserInfo, domain::UserInfos, response::UserInfosError};

/// An authenticated Web client to access the api
///
/// This client will use the private API to fetch data and proceed actions.
#[derive(PartialEq, Debug)]
pub struct AuthenticatedClient {
    base_client: Client,
    csrf_token: Option<String>,
    init_csrf_token: Option<String>,
    rollout_hash: Option<String>,
}

impl AuthenticatedClient {
    /// # Errors
    ///
    /// Will return `Err` if login informations isnt validated.
    pub fn from_login_infos(client: Client, _login_info: &LoginInfos) -> Result<Self, Error> {
        Ok(Self {
            base_client: client,
            csrf_token: None,
            init_csrf_token: None,
            rollout_hash: None,
        })
    }
}

#[async_trait]
impl FetchUserInfo for AuthenticatedClient {
    async fn user_infos(&self, username: &str) -> Result<UserInfos, UserInfosError> {
        self.base_client.user_infos(username).await
    }
}
