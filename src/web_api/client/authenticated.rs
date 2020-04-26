use super::Client;
use crate::web_api::domain::LoginInfos;
use crate::web_api::error::ClientError as Error;

/// An authenticated Web client to access the api
///
/// This client will use the private API to fetch data and proceed actions.
#[derive(PartialEq, Debug)]
pub struct AuthenticatedClient {
    api_url: String,
    csrf_token: Option<String>,
    init_csrf_token: Option<String>,
    rollout_hash: Option<String>,
}

impl AuthenticatedClient {
    pub fn from_login_infos(client: &Client, _login_info: &LoginInfos) -> Result<Self, Error> {
        Ok(Self {
            api_url: client.get_api_url().to_owned(),
            csrf_token: None,
            init_csrf_token: None,
            rollout_hash: None,
        })
    }
}
