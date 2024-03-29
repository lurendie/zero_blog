use serde::{Deserialize, Serialize};
use rbatis::{crud, impl_select_page};
use rbatis::rbdc::datetime::DateTime;
//Blog
#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct Blog{
    id :Option<u16>,
    title:String,
    first_picture :String,
    content :String,
    description :String,
    is_published:u8,
    is_recommend:u8,
    is_appreciation:u8,
    is_comment_enabled:u8,
    create_time:DateTime,
    update_time:DateTime,
    views:u16,
    words:u16,
    read_time:u16,
    category_id:u16,
    is_top:u8,
    password:String,
    user_id:u16
}
crud!(Blog {});
impl_select_page!(Blog{select_page() => "`where is_published = 1`"});
impl_select_page!(Blog{select_page_by_name(name:&str) =>"
     if name != null && name != '':
       `where name != #{name}`
     if name == '':
       `where name != ''`"});             