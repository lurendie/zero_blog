/*
 * @Author: lurendie 
 * @Date: 2024-04-06 09:12:34
 * @LastEditors: lurendie
 * @LastEditTime: 2024-04-23 22:57:39
 * @FilePath: \zero_blog\src\models\vo\page_request.rs
 */
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Default, Serialize, Clone, Debug)]
pub struct PageRequest {
    #[serde(rename = "pageNum")]
    page_num: Option<u16>,
    #[serde(rename = "pageSize")]
    page_size: Option<u16>,
    #[serde(rename = "page")]
    page: Option<u16>,
    #[serde(rename = "blogId")]
    blog_id: Option<u16>,
    #[serde(rename = "password")]
    password: Option<String>,
}

impl PageRequest {
    pub fn get_page_num(&self) -> u16 {
        self.page_num.unwrap_or_default()
    }
    pub fn get_page_size(&self) -> u16 {
        self.page_size.unwrap_or_default()
    }
    pub fn get_blog_id(&self) -> u16 {
        self.blog_id.unwrap_or_default()
    }
    pub fn get_page(&self) -> u16 {
        self.page.unwrap_or_default()
    }
    pub fn get_password(&self) -> String {
        self.password.clone().unwrap_or_default()
    }
}
