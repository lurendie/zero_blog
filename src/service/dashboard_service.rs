use rbs::value::map::ValueMap;

use crate::service::{blog_service, category_service, comments_service};

use super::tag_service;
/**
 * DashboardService 仪表盘
 */
pub struct DashboardService;

impl DashboardService {
    /**
     * 获取博文总数
     */
    pub async fn get_blog_count() -> i32 {
        blog_service::get_blog_count().await
    }
    /**
     * 获取评论总数
     */
    pub async fn get_comment_count() -> i32 {
        comments_service::get_comment_count().await
    }
    /**
     * 获取分类博文数量
     */
    pub async fn get_categorys_count() -> ValueMap {
        //获取分类博文数量
        category_service::get_categorys_count().await
    }

    /**
     * 获取标签博文数量
     */
    pub async fn get_tags_count() -> ValueMap {
        //获取分类博文数量
        tag_service::get_tags_count().await
    }
}
