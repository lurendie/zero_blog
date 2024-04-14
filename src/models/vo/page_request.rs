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
}

impl PageRequest {
    pub fn page_num(&self) -> u16 {
        self.page_num.unwrap_or_default()
    }
    pub fn page_size(&self) -> u16 {
        self.page_size.unwrap_or_default()
    }
    pub fn blog_id(&self) -> u16 {
        self.blog_id.unwrap_or_default()
    }
    pub fn page(&self) -> u16 {
        self.page.unwrap_or_default()
    }
}
