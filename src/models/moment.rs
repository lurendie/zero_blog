use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use crate::entity::moment;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Moment {
    pub(crate) id: Option<u64>,
    pub(crate) content: String,
    #[serde(rename(serialize = "createTime"))]
    pub(crate) create_time: NaiveDateTime,
    pub(crate) likes: i32,
    #[serde(rename(serialize = "published"))]
    pub(crate) is_published: bool,
}

impl From<moment::Model> for Moment {
    fn from(value: moment::Model) -> Self {
        Self {
            id: Some(value.id as u64),
            content: value.content, 
            create_time: value.create_time,
            likes: value.likes.unwrap_or(0),
            is_published: value.is_published,
        }
    }
}
