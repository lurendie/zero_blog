use rbatis::rbdc::datetime::DateTime;
use rbatis::{crud, impl_select_page};
use serde::de::Unexpected;
use serde::{Deserialize, Deserializer, Serialize};

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
    is_comment_enabled: bool,
    create_time: DateTime,
    update_time: DateTime,
    views: u16,
    words: u16,
    read_time: u16,
    category_id: u16,
    #[serde(deserialize_with = "bool_from_int")]
    is_top: bool,
    password: String,
    user_id: u16,
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
