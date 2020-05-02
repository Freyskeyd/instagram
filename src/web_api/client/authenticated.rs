use crate::web_api::{
    behaviour::FetchUserInfos, domain::LoginInfos, domain::UserInfos, error::ClientError,
    response::UserInfosError, Client,
};

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
    pub fn from_login_infos(client: Client, _login_info: &LoginInfos) -> Result<Self, ClientError> {
        Ok(Self {
            base_client: client,
            csrf_token: None,
            init_csrf_token: None,
            rollout_hash: None,
        })
    }
}

#[async_trait::async_trait]
impl FetchUserInfos for AuthenticatedClient {
    async fn fetch_user_infos(&self, username: &str) -> Result<UserInfos, UserInfosError> {
        self.base_client.fetch_user_infos(username).await
    }
}
