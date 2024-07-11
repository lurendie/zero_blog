use rbatis::executor::RBatisTxExecutor;

use crate::models::blog_tag::BlogTag;

pub struct BlogTagService;

impl BlogTagService {
    /**
     * 添加标签
     */
    pub async fn inser_tags(tags: Vec<u16>, blog_id: u16, tx: &mut RBatisTxExecutor) -> bool {
        for tag_id in tags {
            let blog_tag = BlogTag::from(blog_id, tag_id);
            let ok = BlogTag::insert(tx, &blog_tag).await.is_ok();
            if !ok {
                return false;
            }
        }
        true
    }

    /**
     * 删除标签
     */
    pub async fn remove_tags(tags: Vec<u16>, blog_id: u16, tx: &mut RBatisTxExecutor) -> bool {
        for tag_id in tags {
            let blog_tag = BlogTag::from(blog_id, tag_id);
            let ok = BlogTag::delete_by_column(tx, "tag_id", blog_tag.get_tag_id())
                .await
                .is_ok();
            if !ok {
                return false;
            }
        }
        true
    }
}
