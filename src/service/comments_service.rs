use rbatis::{IPage, Page};

use crate::{dao::comment_dao, models::vo::comment::Comment};
//每页显示5条博客简介
const PAGE_SIZE: u64 = 5;
//分页评论
pub(crate) async fn get_comments_page(page_num: usize, page: u16, blog_id: u16) -> Page<Comment> {
    let mut page = comment_dao::get_comments_page(page_num, PAGE_SIZE, page, blog_id)
        .await
        .unwrap_or_else(|e| {
            log::error!("get_comments_page 异常:{e}");
            Page::new(0, 0)
        });
    //获取当前对象子评论
    for item in page.get_records_mut() {
        item.reply_comments=Some(comment_dao::get_comments(item.id.unwrap()).await.unwrap());
    }
    page
}

pub(crate) async fn get_all_comments(blog_id: u16) -> String {
    comment_dao::get_all_comments(blog_id).await.to_string()
}

pub(crate) async fn get_close_comments(blog_id: u16) -> String {
    comment_dao::get_close_comments(blog_id).await.to_string()
}
