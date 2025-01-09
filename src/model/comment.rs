use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
//评论
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub(crate) id: Option<i64>,
    pub(crate) nickname: String,
    pub(crate) email: String,
    pub(crate) content: String,
    pub(crate) avatar: String,
    pub(crate) create_time: NaiveDateTime,
    pub(crate) ip: String,
  
    pub(crate) is_published: bool,
  
    pub(crate) is_admin_comment: bool,
    pub(crate) page: u16,
    pub(crate) is_notice: bool,
    pub(crate) blog_id: Option<i64>,
    pub(crate) parent_comment_id: Option<i64>,
    pub(crate) website: Option<String>,
    pub(crate) qq: Option<String>,
    pub(crate) comments: Option<Vec<Comment>>,
}

