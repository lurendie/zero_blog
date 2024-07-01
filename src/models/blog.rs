use rbatis::rbdc::datetime::DateTime;
use rbatis::{crud, impl_select, impl_select_page};
use serde::de::Unexpected;
use serde::{Deserialize, Deserializer, Serialize};

use crate::dao::CategoryDao;

use super::category::Category;

//Blog
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Blog {
    id: Option<u16>,
    title: String,
    first_picture: String,
    content: String,
    description: String,
    #[serde(deserialize_with = "bool_from_int")]
    is_published: bool,
    #[serde(deserialize_with = "bool_from_int")]
    is_recommend: bool,
    #[serde(deserialize_with = "bool_from_int")]
    is_appreciation: bool,
    #[serde(deserialize_with = "bool_from_int")]
    is_comment_enabled: bool,
    create_time: DateTime,
    update_time: DateTime,
    views: u16,
    words: u16,
    read_time: u16,
    //category_id: u16,
    #[serde(deserialize_with = "bool_from_int")]
    is_top: bool,
    password: Option<String>,
    user_id: u16,
    #[serde(rename(deserialize = "category_id"), skip_serializing)]
    //跳过该字段，不进行序列化操作。
    pub(crate) category_id: u16,
    //#[serde(skip_deserializing)] // 跳过该字段，不进行反序列化操作。
    pub(crate) category: Option<Category>,
}

// int 类型转boolean
pub fn bool_from_int<'de, D>(deserializer: D) -> Result<bool, D::Error>
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

crud!(Blog {});
impl_select_page!(Blog{select_page() => "`where is_published = 1`"});
impl_select_page!(Blog{select_page_by_name(name:&str) =>"
     if name != null && name != '':
       `where name != #{name}`
     if name == '':
       `where name != ''`"});

impl_select_page!(Blog{select_page_blog_all(title:&str) =>"where 1=1
if !title.is_empty():
   `and title like #{title}`"});

impl_select!(Blog{get_blog(id:&str)=>"`where blog.id = #{id}`"});

// // id 类型转 category
fn _category_from_id<'de, D>(deserializer: D) -> Result<Option<Category>, D::Error>
where
    D: Deserializer<'de>,
{
    if let Ok(id) = u16::deserialize(deserializer) {
        let fut = CategoryDao::get_by_id(id as u16);
        let _v = Box::pin(async { Some(fut.await.unwrap()) });
        return Ok(None);
    }
    Ok(Some(Category::default()))
}
