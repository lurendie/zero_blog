use crate::models::category::Category;
use rbatis::rbdc::datetime::DateTime;
use rbatis::{crud,impl_select};
use serde::{Serialize,Deserialize};
//博客详情信息
#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct BlogDetail{
    id :Option<u16>,
    title:String,
    content :String,
    is_appreciation:u8,
    is_comment_enabled:u8,
    create_time:DateTime,
    update_time:DateTime,
    views:u16,
    words:u16,
    read_time:u16,
    is_top:u8,
    // todo 结构体引用
    category:Option<Category>,
    password:String,

}

crud!(BlogDetail {},"blog");
impl_select!(BlogDetail{select_by_id(id:String) -> Option => "`where id = #{id} limit 1`"},"blog");