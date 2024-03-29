use crate::models::{tag::Tag,category::Category};
use rbatis::{crud, impl_select_page};
use serde::{Serialize,Deserialize};
//use rbatis::rbdc::datetime::DateTime;
//博客简要信息
#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct BlogInfo{
    pub id :Option<u16>,
    pub title:String,
    pub description :String,
    #[serde(rename(serialize ="createTime"))]
    pub create_time:String,
    pub views:u16,
    pub words:u16,
    pub read_time:u16,
    pub password:Option<String>,
    pub privacy:Option<bool>,
    pub is_top:u16,
    pub tags:Option<Vec<Tag>>,
    pub category:Option<Category>,
}

crud!(BlogInfo {},"blog");
impl_select_page!(BlogInfo{select_page()=>"
      if !sql.contains('count(1)'):
      `order by create_time desc`"},"blog");
impl_select_page!(BlogInfo{select_page_by_categoryid(id:&str) =>"
     if id != null && id != '':
       `where category_id = #{id}`"},"blog");
