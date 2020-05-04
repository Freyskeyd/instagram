pub(crate) mod client;
pub use client::AuthenticatedClient;
pub use client::Client;

mod credentials;
pub use credentials::Credentials;

pub mod domain;

pub mod error;
pub use error::ClientError;

pub mod response;

pub mod behaviour {
    use async_trait::async_trait;
    use serde::Serialize;

    use super::domain::{UserFeed, UserInfos};
    use super::error::ClientError;
    use super::response::UserInfosError;

    pub struct PaginationOptions<'a> {
        pub count: i16,
        pub after: Option<&'a str>,
    }

    impl<'a> std::default::Default for PaginationOptions<'a> {
        fn default() -> Self {
            Self {
                count: 12,
                after: None,
            }
        }
    }

    #[derive(Debug, Serialize)]
    pub struct FetchUserFeedOptions<'a, 'b> {
        user_id: Option<&'a str>,
        count: i16,
        after: Option<&'b str>,
    }

    impl<'a, 'b> FetchUserFeedOptions<'a, 'b> {
        #[must_use]
        pub fn set_user_id(mut self, user_id: &'a str) -> Self {
            self.user_id = Some(user_id);

            self
        }
    }
    impl<'a, 'b> std::default::Default for FetchUserFeedOptions<'a, 'b> {
        fn default() -> Self {
            Self {
                count: 12,
                after: None,
                user_id: None,
            }
        }
    }

    #[async_trait]
    pub trait FetchUserInfos {
        /// Fetch user's informations
        ///
        /// # Examples
        ///
        /// ```rust
        /// use instagram::web_api::Client;
        /// use instagram::web_api::domain::UserInfos;
        /// use instagram::web_api::behaviour::FetchUserInfos;
        /// # async fn doc() -> Result<(), reqwest::Error> {
        /// let client = Client::new();
        ///
        /// let some_user_info: UserInfos = client.fetch_user_infos("SomeUser").await.expect("Unable to
        /// retriev user infos");
        /// # Ok(())
        /// # }
        /// ```
        ///
        /// # Errors
        ///
        /// Will return `Err` if the request fails on instagram api.
        async fn fetch_user_infos(&self, username: &str) -> Result<UserInfos, UserInfosError>;
    }

    #[async_trait]
    pub trait FetchUserFeed {
        /// Fetch user's feed
        ///
        /// # Examples
        ///
        /// ```rust
        /// use instagram::web_api::Client;
        /// use instagram::web_api::domain::UserFeed;
        /// use instagram::web_api::behaviour::FetchUserFeed;
        /// # async fn doc() -> Result<(), reqwest::Error> {
        /// let client = Client::new();
        ///
        /// let user_id = "some_id";
        /// let some_user_feed: UserFeed = client.fetch_user_feed(&user_id, None).await.expect("Unable to
        /// retriev user feed");
        /// # Ok(())
        /// # }
        /// ```
        ///
        /// # Errors
        ///
        /// Will return `Err` if the request fails on instagram api.
        async fn fetch_user_feed(
            &self,
            user_id: &str,
            options: Option<FetchUserFeedOptions<'_, '_>>,
        ) -> Result<UserFeed, ClientError>;
    }

    #[async_trait]
    pub trait FetchMediaInfo {
        async fn fetch_media_infos(&self, username: &str) -> Result<(), ClientError>;
    }

    pub trait FetchMediaComments {}
    pub trait Search {}
    pub trait FetchTagFeed {}
    pub trait FetchLocationFeed {}
    pub trait FetchStoryFeed {} // Not Logged in?
    pub trait FetchHighlightReelMedia {}
    pub trait FetchTaggedUserFeed {}
    pub trait FetchTagStoryFeed {}
    pub trait FetchLocationStoryFeed {}

    // require login
    pub trait FetchMediaLikers {}
    pub trait FetchUserFollowing {}
    pub trait FetchUserFollowers {}
    pub trait LikePost {}
    pub trait CancelPostLike {}
    pub trait DeleteMedia {}
    pub trait Follow {}
    pub trait UnFollow {}
    pub trait CommentPost {}
    pub trait DeleteComment {}
    pub trait PostPhoto {}
    pub trait FetchTimelineFeed {}
    pub trait FetchReelsTray {}
    pub trait FetchReelsFeed {}
    pub trait FetchHighlightReels {}
}
