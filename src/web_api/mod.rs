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

    use super::domain::UserInfos;
    use super::response::UserInfosError;

    #[async_trait]
    pub trait FetchUserInfo {
        async fn user_infos(&self, username: &str) -> Result<UserInfos, UserInfosError>;
    }

    pub trait FetchUserFeed {}
    pub trait FetchMediaInfo {}
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
