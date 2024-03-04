use crate::models::category::Category;
use rbatis::rbdc::datetime::DateTime;
use rbatis::{crud,impl_select};
use serde::{Serialize,Deserialize};
//博客详情信息
#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct BlogDetail{
    pub(crate) id :Option<u16>,
    pub(crate) title:String,
    pub(crate) content :String,
    pub(crate) is_appreciation:u8,
    pub(crate) is_comment_enabled:u8,
    pub(crate) create_time:DateTime,
    pub(crate) update_time:DateTime,
    pub(crate) views:u16,
    pub(crate) words:u16,
    pub(crate) read_time:u16,
    pub(crate) is_top:u8,
    // todo 结构体引用
    pub(crate) category:Option<Category>,
    pub(crate) password:String,

}

impl BlogDetail{
    pub(crate) fn new ()-> Self{
        BlogDetail{
            id:None,
            title:"".to_string(),
            content:"".to_string(),
            is_appreciation:0,
            is_comment_enabled:0,
            create_time:DateTime::now(),
            update_time:DateTime::now(),
            views:0,
            words:0,
            read_time:0,
            is_top:0,
            category:None,
            password:"".to_string(),
        }
    }
}

crud!(BlogDetail {},"blog");
impl_select!(BlogDetail{select_by_id(id:String) -> Option => "`where id = #{id} limit 1`"},"blog");