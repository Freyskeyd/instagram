use serde::Deserialize;

mod deserializer;
mod infos;

pub use infos::LoginInfos;
pub use infos::UserInfos;

#[derive(Debug, Deserialize)]
pub struct UserFeed {
    pub count: i32,
    #[serde(rename = "edges", deserialize_with = "deserializer::nested_media")]
    pub medias: Vec<Media>,
    #[serde(rename = "page_info")]
    pub pagination_infos: PaginationInfos,
}

#[derive(Debug, Deserialize)]
pub struct PaginationInfos {
    end_cursor: Option<String>,
    has_next_page: bool,
}

#[derive(Debug, Deserialize)]
pub struct MediaDimensions {
    height: i32,
    width: i32,
}

#[derive(Debug, Deserialize)]
pub struct MediaComments {
    count: i32,
    #[serde(
        rename = "edges",
        deserialize_with = "deserializer::nested_media_comment"
    )]
    data: Vec<MediaComment>,
    #[serde(rename = "page_info")]
    pagination_infos: PaginationInfos,
}

#[derive(Debug, Deserialize)]
pub struct MediaComment {
    id: String,
    created_at: i32,
    did_report_as_spam: bool,
    text: String,
    viewer_has_liked: bool,
    owner: CommentUser,
}

#[derive(Debug, Deserialize)]
pub struct CommentUser {
    id: String,
    is_verified: bool,
    profile_pic_url: String,
    username: String,
}

#[derive(Debug, Deserialize)]
pub struct MediaOwner {
    id: String,
    username: String,
}

#[derive(Debug, Deserialize)]
pub struct ThumbnailResource {
    src: String,
    #[serde(rename = "config_height")]
    height: i32,
    #[serde(rename = "config_width")]
    width: i32,
}

#[derive(Debug, Deserialize)]
pub struct Location {
    id: String,
    has_public_page: bool,
    name: String,
    slug: String,
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Deserialize)]
pub struct Media {
    id: String,

    #[serde(
        rename = "edge_media_to_caption",
        deserialize_with = "deserializer::nested_media_caption"
    )]
    caption: Option<String>,

    #[serde(rename = "edge_media_to_comment")]
    comments: MediaComments,

    comments_disabled: bool,
    dimensions: MediaDimensions,
    display_url: String,

    #[serde(
        rename = "edge_media_preview_like",
        deserialize_with = "deserializer::nested_media_likes"
    )]
    like: i32,
    is_video: bool,
    // edge_media_to_sponsor_user: {edges: []}
    // edge_media_to_tagged_user: {edges: []}
    // user_tags: MediaUserTags,
    // fact_check_information: null
    // fact_check_overall_rating: null
    // gating_info: null
    // location: Location,
    // media_overlay_info: null
    media_preview: Option<String>,
    owner: MediaOwner,
    shortcode: String,
    taken_at_timestamp: i64,

    #[serde(rename = "thumbnail_resources")]
    thumbnails: Vec<ThumbnailResource>,

    thumbnail_src: String,
    tracking_token: String,
    viewer_can_reshare: bool,
    viewer_has_liked: bool,
    viewer_has_saved: bool,
    viewer_has_saved_to_collection: bool,
    viewer_in_photo_of_you: bool,
}
