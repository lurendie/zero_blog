use rbatis::{crud, impl_select_page};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagVO {
    pub id: Option<u16>,
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
crud!(TagVO {}, "tag");
impl_select_page!(TagVO{get_tags_by_page()=>""}, "tag");
