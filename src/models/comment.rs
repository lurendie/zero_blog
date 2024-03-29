use rbatis::{crud, impl_select_page};
use serde::{Deserialize, Serialize};
//评论
#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct Comment{
    id :Option<u16>,
    nickname:String,
    email:String,
    content:String,
    avatar:String,
    create_time:String,
    ip:String,
    is_published:u8,
    is_admin_comment:u8,
    page:u16,
    is_notice:u8,
    blog_id:Option<u16>,
    parent_comment_id:Option<u16>,
    website:String,
    qq:String,
}
crud!(Comment{});
//分页查询
impl_select_page!(Comment{select_page() => ""});