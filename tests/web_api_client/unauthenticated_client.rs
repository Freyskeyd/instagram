extern crate instagram;

use instagram::web_api::Client;

use instagram::web_api::behaviour::*;

#[test]
fn behaviour() {
    assert_impl!(FetchUserInfos: Client);
    assert_impl!(FetchUserFeed: Client);

    // TODO implementation needed
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
}
