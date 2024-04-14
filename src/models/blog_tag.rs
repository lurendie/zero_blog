use rbatis::crud;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct BlogTag{
    blog_id:Option<u16>,
    tag_id:u16
}
crud!(BlogTag {});