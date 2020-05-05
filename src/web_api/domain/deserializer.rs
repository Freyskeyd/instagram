use super::{Media, MediaComment};
use serde::Deserialize;
use serde::Deserializer;

pub fn nested_media_caption<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Debug, Deserialize)]
    pub struct MediaCaptionList {
        edges: Vec<MediaCaption>,
    }

    #[derive(Debug, Deserialize)]
    pub struct MediaCaption {
        node: MediaCaptionText,
    }

    #[derive(Debug, Deserialize)]
    pub struct MediaCaptionText {
        text: String,
    }

    MediaCaptionList::deserialize(deserializer)
        .map(|mut a| a.edges.pop())
        .map(|x| x.map(|x| x.node.text))
}

pub fn nested_media_likes<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Debug, Deserialize)]
    pub struct MediaLike {
        count: i32,
    }

    MediaLike::deserialize(deserializer).map(|a| a.count)
}

pub fn nested_media<'de, D>(deserializer: D) -> Result<Vec<Media>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Wrapper(MediaEdge);

    #[derive(Debug, Deserialize)]
    pub struct MediaEdge {
        node: Media,
    }

    let v = Vec::deserialize(deserializer)?;

    Ok(v.into_iter()
        .map(|Wrapper(a)| a.node)
        .collect::<Vec<Media>>())
}

pub fn nested_media_comment<'de, D>(deserializer: D) -> Result<Vec<MediaComment>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Wrapper(MediaEdge);

    #[derive(Debug, Deserialize)]
    pub struct MediaEdge {
        node: MediaComment,
    }

    let v = Vec::deserialize(deserializer)?;

    Ok(v.into_iter()
        .map(|Wrapper(a)| a.node)
        .collect::<Vec<MediaComment>>())
}
