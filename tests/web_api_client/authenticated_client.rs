extern crate instagram;

use instagram::web_api::AuthenticatedClient as Client;

use instagram::web_api::behaviour::*;

#[test]
fn behaviour() {
    assert_impl!(FetchUserInfo: Client);
    // TODO implementation needed
    assert_impl!(!FetchUserFeed: Client);
    assert_impl!(!FetchMediaInfo: Client);
    assert_impl!(!FetchMediaComments: Client);
    assert_impl!(!Search: Client);
    assert_impl!(!FetchTagFeed: Client);
    assert_impl!(!FetchLocationFeed: Client);
    assert_impl!(!FetchStoryFeed: Client);
    assert_impl!(!FetchHighlightReelMedia: Client);
    assert_impl!(!FetchTaggedUserFeed: Client);
    assert_impl!(!FetchTagStoryFeed: Client);
    assert_impl!(!FetchLocationStoryFeed: Client);
    assert_impl!(!FetchMediaLikers: Client);
    assert_impl!(!FetchUserFollowing: Client);
    assert_impl!(!FetchUserFollowers: Client);
    assert_impl!(!LikePost: Client);
    assert_impl!(!CancelPostLike: Client);
    assert_impl!(!DeleteMedia: Client);
    assert_impl!(!Follow: Client);
    assert_impl!(!UnFollow: Client);
    assert_impl!(!CommentPost: Client);
    assert_impl!(!DeleteComment: Client);
    assert_impl!(!PostPhoto: Client);
    assert_impl!(!FetchTimelineFeed: Client);
    assert_impl!(!FetchReelsTray: Client);
    assert_impl!(!FetchReelsFeed: Client);
    assert_impl!(!FetchHighlightReels: Client);
}
