
use serde::{Deserialize, Serialize};
use crate::entity::tag;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagVO {
    pub id: Option<i64>,
    #[serde(rename(deserialize = "tag_name"))]
    pub name: String,
    pub color: String,
}

impl Default for TagVO {
    fn default() -> Self {
        Self {
            id: Some(0),
            name: "未知".to_string(),
            color: "#000000".to_string(),
        }
    }
}

impl From<tag::Model> for TagVO {
    fn from(t: tag::Model) -> Self {
        Self {
            id: Some(t.id),
            name: t.tag_name,
            color: t.color.unwrap_or("#000000".to_string()),
        }
    }
}
