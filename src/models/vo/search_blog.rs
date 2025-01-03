use crate::entity::blog;
use serde::{Deserialize, Serialize};
/**
 * 文章搜索
 */
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SearchBlog {
    #[serde(rename(serialize = "id"))]
    id: i64,
    title: String,
    content: String,
}
impl SearchBlog {
    pub fn get_id(&self) -> i64 {
        self.id
    }
    pub fn set_id(&mut self, id: i64) {
        self.id = id;
    }
    pub fn get_title(&self) -> String {
        self.title.clone()
    }
    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }
    pub fn get_content(&self) -> String {
        self.content.clone()
    }
    pub fn set_content(&mut self, content: String) {
        self.content = content;
    }
}

impl From<blog::Model> for SearchBlog {
    fn from(b: blog::Model) -> Self {
        Self {
            id: b.id,
            title: b.title,
            content: b.content,
        }
    }
}
