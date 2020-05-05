use super::UserFeed;
use serde::{Deserialize, Deserializer};

pub fn nested_user_feed<'de, D>(deserializer: D) -> Result<UserFeed, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct A {
        edge_owner_to_timeline_media: UserFeed,
    }

    A::deserialize(deserializer).map(|a| a.edge_owner_to_timeline_media)
}
