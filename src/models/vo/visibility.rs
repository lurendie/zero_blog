use serde::{Deserialize, Serialize};

//Blog可见性
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Visibility {
    appreciation: Option<bool>, // 赞赏可见性，默认为false。如果设置为false，则赞赏不可见。
    #[serde(rename = "commentEnabled")]
    comment_enabled: Option<bool>, // 评论可见性，默认为false。如果设置为false，则评论不可见。
    password: Option<String>,   // 密码，如果设置，则只有知道密码的人才能看到帖子。
    published: Option<bool>,    // 可见性，默认为false。如果设置为false，则分享不可见。
    recommended: Option<bool>,  // 推荐可见性，默认为false。如果设置为false，则推荐不可见。
    top: Option<bool>,          // 置顶可见性，默认为false。如果设置为false，则置顶不可见。
}

impl Visibility {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn set_appreciation(&mut self, appreciation: bool) -> &mut Self {
        self.appreciation = Some(appreciation);
        self
    }
    pub fn set_comment_enabled(&mut self, comment_enabled: bool) -> &mut Self {
        self.comment_enabled = Some(comment_enabled);
        self
    }
    pub fn set_password(&mut self, password: String) -> &mut Self {
        self.password = Some(password);
        self
    }
    pub fn set_published(&mut self, published: bool) -> &mut Self {
        self.published = Some(published);
        self
    }
    pub fn set_recommended(&mut self, recommended: bool) -> &mut Self {
        self.recommended = Some(recommended);
        self
    }
    pub fn set_top(&mut self, top: bool) -> &mut Self {
        self.top = Some(top);
        self
    }
    pub fn get_appreciation(&self) -> Option<bool> {
        self.appreciation
    }
    pub fn get_comment_enabled(&self) -> Option<bool> {
        self.comment_enabled
    }
    pub fn get_password(&self) -> Option<String> {
        self.password.clone()
    }
    pub fn get_published(&self) -> Option<bool> {
        self.published
    }
    pub fn get_recommended(&self) -> Option<bool> {
        self.recommended
    }
    pub fn get_top(&self) -> Option<bool> {
        self.top
    }
}