use crate::service::CommonService;
use rbatis::rbdc::datetime::DateTime;
use rbatis::{crud, impl_select, impl_select_page, impl_update};
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
    #[serde(deserialize_with = "CommonService::bool_from_int")]
    is_published: bool,
    #[serde(deserialize_with = "CommonService::bool_from_int")]
    is_recommend: bool,
    #[serde(deserialize_with = "CommonService::bool_from_int")]
    is_appreciation: bool,
    #[serde(deserialize_with = "CommonService::bool_from_int")]
    is_comment_enabled: bool,
    create_time: DateTime,
    update_time: DateTime,
    views: u16,
    words: u16,
    read_time: u16,
    //category_id: u16,
    #[serde(deserialize_with = "CommonService::bool_from_int")]
    is_top: bool,
    password: Option<String>,
    user_id: u16,
    #[serde(rename(deserialize = "category_id"), skip_serializing)]
    //跳过该字段，不进行序列化操作。
    category_id: u16,
    //#[serde(skip_deserializing)] // 跳过该字段，不进行反序列化操作。
    category: Option<Category>,
}

crud!(Blog {});
impl_update!(Blog{update_by_id(id:&str) => "`where id = #{id}`"},"blog");
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

impl Blog {
    pub fn get_id(&self) -> u16 {
        self.id.unwrap_or(0)
    }
    pub fn get_title(&self) -> &str {
        &self.title
    }
    pub fn get_first_picture(&self) -> &str {
        &self.first_picture
    }

    pub fn get_content(&self) -> &str {
        &self.content
    }
    pub fn get_description(&self) -> &str {
        &self.description
    }
    pub fn get_is_published(&self) -> bool {
        self.is_published
    }
    pub fn get_is_recommend(&self) -> bool {
        self.is_recommend
    }
    pub fn get_is_appreciation(&self) -> bool {
        self.is_appreciation
    }
    pub fn get_is_comment_enabled(&self) -> bool {
        self.is_comment_enabled
    }

    pub fn get_views(&self) -> u16 {
        self.views
    }
    pub fn get_words(&self) -> u16 {
        self.words
    }
    pub fn get_read_time(&self) -> u16 {
        self.read_time
    }
    pub fn get_is_top(&self) -> bool {
        self.is_top
    }
    pub fn get_password(&self) -> Option<&str> {
        self.password.as_deref()
    }
    pub fn get_user_id(&self) -> u16 {
        self.user_id
    }
    pub fn get_category_id(&self) -> u16 {
        self.category_id
    }
    pub fn get_category(&self) -> Option<Category> {
        self.category.clone()
    }

    pub fn set_id(&mut self, id: u16) -> &mut Self {
        self.id = Some(id);
        self
    }
    pub fn set_title(&mut self, title: &str) -> &mut Self {
        self.title = title.to_string();
        self
    }
    pub fn set_first_picture(&mut self, first_picture: &str) -> &mut Self {
        self.first_picture = first_picture.to_string();
        self
    }
    pub fn set_content(&mut self, content: &str) -> &mut Self {
        self.content = content.to_string();
        self
    }
    pub fn set_description(&mut self, description: &str) -> &mut Self {
        self.description = description.to_string();
        self
    }
    pub fn set_is_published(&mut self, is_published: bool) -> &mut Self {
        self.is_published = is_published;
        self
    }
    pub fn set_is_recommend(&mut self, is_recommend: bool) -> &mut Self {
        self.is_recommend = is_recommend;
        self
    }
    pub fn set_is_appreciation(&mut self, is_appreciation: bool) -> &mut Self {
        self.is_appreciation = is_appreciation;
        self
    }
    pub fn set_is_comment_enabled(&mut self, is_comment_enabled: bool) -> &mut Self {
        self.is_comment_enabled = is_comment_enabled;
        self
    }
    pub fn set_create_time(&mut self, create_time: DateTime) -> &mut Self {
        self.create_time = create_time;
        self
    }
    pub fn set_update_time(&mut self, update_time: DateTime) -> &mut Self {
        self.update_time = update_time;
        self
    }
    pub fn set_views(&mut self, views: u16) -> &mut Self {
        self.views = views;
        self
    }
    pub fn set_words(&mut self, words: u16) -> &mut Self {
        self.words = words;
        self
    }
    pub fn set_read_time(&mut self, read_time: u16) -> &mut Self {
        self.read_time = read_time;
        self
    }
    pub fn set_is_top(&mut self, is_top: bool) -> &mut Self {
        self.is_top = is_top;
        self
    }
    pub fn set_password(&mut self, password: Option<&str>) -> &mut Self {
        self.password = password.map(|s| s.to_string());
        self
    }
    pub fn set_user_id(&mut self, user_id: u16) -> &mut Self {
        self.user_id = user_id;
        self
    }
    pub fn set_category_id(&mut self, category_id: u16) -> &mut Self {
        self.category_id = category_id;
        self
    }
    pub fn set_category(&mut self, category: Option<Category>) -> &mut Self {
        self.category = category.clone();
        self
    }
}
