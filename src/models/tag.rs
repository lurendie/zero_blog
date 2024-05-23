use rbatis::crud;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub id: Option<u16>,
    pub name: String,
    pub color: String,
}

impl Default for Tag {
    fn default() -> Self {
        Self {
            id: Some(0),
            name: "未知".to_string(),
            color: "#000000".to_string(),
        }
    }
}
crud!(Tag {});
