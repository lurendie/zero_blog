use rbatis::rbdc::Error;
use rbatis::{Page, PageRequest};

use crate::models::comment::Comment;
use crate::rbatis::RBATIS;

//获取评论
pub(crate) async fn get_comments_page(
    page_num: usize,
    page_size: u64,
) -> Result<Page<Comment>, Error> {
    Comment::select_page(
        &RBATIS.acquire().await.unwrap(),
        &PageRequest::new(page_num as u64, page_size),
    )
    .await
}
