use crate::models::{tag::Tag,category::Category};
use rbatis::{crud, impl_select_page};
use rbs;
use serde::{Serialize,Deserialize};
use rbatis::rbdc::datetime::DateTime;
//博客简要信息
#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct BlogInfo{
    pub id :Option<u16>,
    pub title:String,
    pub description :String,
    pub create_time:DateTime,
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

crud!(BlogInfo {},"blog");
impl_select_page!(BlogInfo{select_page()=>"
      if !sql.contains('count(1)'):
      `order by create_time desc`"},"blog");
impl_select_page!(BlogInfo{select_page_by_categoryid(id:&str) =>"
     if id != null && id != '':
       `where category_id = #{id}`"},"blog");
