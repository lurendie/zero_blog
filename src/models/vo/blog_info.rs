use crate::models::{category::Category, tag::Tag};
use rbatis::{crud, impl_select_page};
use serde::{de::Unexpected, Deserialize, Deserializer, Serialize};
//use rbatis::rbdc::datetime::DateTime;
//博客简要信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BlogInfo {
    pub id: Option<u16>,
    pub title: String,
    pub description: String,
    #[serde(rename(serialize = "createTime"))]
    pub create_time: String,
    pub views: u16,
    pub words: u16,
    pub read_time: u16,
    pub password: Option<String>,
    pub privacy: Option<bool>,
    #[serde(deserialize_with = "bool_from_int")]
    pub is_top: bool,
    pub tags: Option<Vec<Tag>>,
    pub category: Option<Category>,
}

// impl BlogInfo {
//     fn new() -> Self {
//         BlogInfo::default()
//     }
// }

fn bool_from_int<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match u64::deserialize(deserializer)? {
        0 => Ok(false),
        1 => Ok(true),
        other => Err(serde::de::Error::invalid_value(
            Unexpected::Unsigned(other),
            &"0 or 1",
        )),
    }
}

crud!(BlogInfo {}, "blog");
impl_select_page!(BlogInfo{select_page()=>"
      if !sql.contains('count(1)'):
      `order by is_top desc, create_time desc`"},"blog");
impl_select_page!(BlogInfo{select_page_by_categoryid(id:&str) =>"
     if id != null && id != '':
       `where category_id = #{id}`"},"blog");
