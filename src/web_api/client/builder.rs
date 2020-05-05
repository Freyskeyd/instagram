use crate::web_api::Client;

pub struct ClientBuilder<'a, 'b> {
    url: &'a str,
    graphql_api_url: &'b str,
}

impl<'a, 'b> ClientBuilder<'a, 'b> {
    pub const fn new() -> Self {
        Self {
            url: "https://www.instagram.com",
            graphql_api_url: "https://www.instagram.com/graphql/query",
        }
    }

    pub const fn set_api_url(mut self, url: &'a str) -> Self {
        self.url = url;

        self
    }

    pub const fn set_graphql_api_url(mut self, url: &'b str) -> Self {
        self.graphql_api_url = url;

        self
    }

    pub fn build(self) -> Client {
        Client {
            api_url: self.url.to_string(),
            graphql_api_url: self.graphql_api_url.to_string(),
            csrf_token: None,
            init_csrf_token: None,
            rollout_hash: None,
        }
    }
}
