use async_trait::async_trait;

use crate::web_api::{
    domain::{UserFeed, UserInfos},
    error::ClientError,
    options::FetchUserFeedOptions,
    response::UserInfosError,
};

#[async_trait]
pub trait FetchUserInfos {
    /// Fetch user's informations
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use instagram::web_api::Client;
    /// # use instagram::web_api::domain::UserInfos;
    /// # use instagram::web_api::behaviour::FetchUserInfos;
    /// # async fn doc() -> Result<(), reqwest::Error> {
    ///
    /// let some_user_info: UserInfos = Client::new().fetch_user_infos("SomeUser").await.expect(
    ///     "Unable to
    /// retriev user infos",
    /// );
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
    /// use instagram::web_api::behaviour::FetchUserFeed;
    /// use instagram::web_api::domain::UserFeed;
    /// use instagram::web_api::Client;
    /// # async fn doc() -> Result<(), reqwest::Error> {
    /// let client = Client::new();
    ///
    /// let user_id = "some_id";
    /// let some_user_feed: UserFeed = client.fetch_user_feed(&user_id, None).await.expect(
    ///     "Unable to
    /// retriev user feed",
    /// );
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
pub trait FetchMediaInfos {
    /// Fetch media's infos
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use instagram::web_api::Client;
    /// use instagram::web_api::behaviour::FetchMediaInfos;
    /// # async fn doc() -> Result<(), reqwest::Error> {
    /// let client = Client::new();
    ///
    /// let media_id = "some_id";
    /// let some_media = client
    ///
    /// .fetch_media_infos(&media_id).await.expect("Unable to
    /// retriev media info");
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Will return `Err` if the request fails on instagram api.
    async fn fetch_media_infos(&self, media_id: &str) -> Result<(), ClientError>;
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
