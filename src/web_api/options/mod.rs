use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct PaginationOptions<'a> {
    #[serde(rename = "first")]
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
    #[serde(rename = "id")]
    user_id: Option<&'a str>,

    #[serde(flatten)]
    pagination: PaginationOptions<'b>,
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
            pagination: PaginationOptions::default(),
            user_id: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FetchUserFeedOptions;
    use serde_json::json;

    #[test]
    fn test_pagination_serialization() {
        let fetch_user_opts = FetchUserFeedOptions::default();

        let expected = json!({
            "id": serde_json::Value::Null,
            "first": 12,
            "after": serde_json::Value::Null
        })
        .to_string();

        assert_eq!(expected, serde_json::to_string(&fetch_user_opts).unwrap());
    }
}
