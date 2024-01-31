use rbatis::crud_table;
use crate::models::{tag::Tag,category::Category};
//博客简要信息
#[derive(Debug, Clone)]
#[crud_table]
pub struct BlogInfo{
    pub id :Option<u16>,
    pub title:String,
    pub description :String,
    pub create_time:chrono::NaiveDateTime,
    pub views:u16,
    pub words:u16,
    pub read_time:u16,
    pub password:String,
    //#[serde(skip_serializing_if = "Option::is_none")]
    pub private:Option<bool>,
    pub is_top:u16,
    //#[serde(skip_serializing_if = "Option::is_none")]
    pub tags:Option<Vec<Tag>>,
    //#[serde(skip_serializing_if = "Option::is_none")]
    pub category:Option<Category>,
}

// impl CRUDTable for BlogInfo{
//     fn table_name() -> String {
//         "blog".to_string()
//     }
//
//     fn table_columns() -> String {
//         "id,title,description,create_time,views,words,is_top,read_time,password".to_string()
//     }
// }
