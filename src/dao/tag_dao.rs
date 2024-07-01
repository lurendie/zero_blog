use crate::models::tag::Tag;
use crate::rbatis::RBATIS;

pub struct TagDao;
impl TagDao {
    pub async fn get_list() -> Result<Vec<Tag>, rbatis::Error> {
        let sql = format!(
            "
            select
            tag.id as id,tag_name as name,color
            from tag
            "
        );
        let tags: Result<Vec<Tag>, rbatis::Error> = RBATIS.query_decode(&sql, vec![]).await;
        tags
    }

    /**
     * 根据BlogId查询当前所有的云标签
     */
    pub async fn get_blog_tags(id: u16) -> Vec<crate::models::tag::Tag> {
        //TagList
        let sql = format!(
            "
            select
            tag.id as id,tag_name as name,color
            from blog_tag,tag
            where blog_tag.tag_id = tag.id and blog_tag.blog_id = {}
            ",
            id
        );
        let tags = RBATIS
            .query_decode::<Vec<Tag>>(&*sql, vec![])
            .await
            .unwrap_or_else(|e| {
                log::error!("异常:{e}");
                vec![]
            });
        tags
    }
}
