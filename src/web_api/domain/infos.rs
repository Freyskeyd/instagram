use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct UserInfos {
    pub biography: String,
    pub blocked_by_viewer: bool,
    pub restricted_by_viewer: Option<bool>,
    pub country_block: bool,
    pub external_url: Option<String>,
    pub external_url_linkshimmed: Option<String>,
    pub followed_by_viewer: bool,
    pub follows_viewer: bool,
    pub full_name: String,
    pub has_ar_effects: bool,
    pub has_channel: bool,
    pub has_blocked_viewer: bool,
    pub highlight_reel_count: i32,
    pub has_requested_viewer: bool,
    pub id: String,
    pub is_business_account: bool,
    pub is_joined_recently: bool,
    pub business_category_name: String,
    pub category_id: String,
    pub overall_category_name: Option<String>,
    pub is_private: bool,
    pub is_verified: bool,
    pub profile_pic_url: String,
    pub profile_pic_url_hd: String,
    pub requested_by_viewer: bool,
    pub username: String,
    pub connected_fb_page: Option<String>,

    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginInfos {
    authenticated: bool,
    user: bool,
    user_id: String,
    one_tap_prompt: bool,
    status: String,
}
