use rbatis::{crud_table};
use serde::{Deserialize, Serialize};

//Blog
#[crud_table(table_name: "blog")]
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
    create_time:chrono::NaiveDateTime,
    update_time:chrono::NaiveDateTime,
    views:u16,
    words:u16,
    read_time:u16,
    category_id:u16,
    is_top:u8,
    password:String,
    user_id:u16
}