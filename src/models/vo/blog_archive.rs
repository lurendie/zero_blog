use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default, FromQueryResult)]
pub(crate) struct BlogArchive {
    pub id: i64,
    pub password: Option<String>,
    pub privacy: Option<bool>,
    pub title: String,
    pub day: Option<i64>,
}

impl BlogArchive {
    pub(crate) fn _new() -> Self {
        BlogArchive {
            id: 0,
            password: Some("".to_string()),
            privacy: Some(false),
            title: "".to_string(),
            day: None,
        }
    }
}
