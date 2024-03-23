use rbatis::{IPage, Page};

use crate::dao::comment;
//每页显示5条博客简介
const PAGE_SIZE:u64= 5;
//分页评论
pub(crate) async fn get_comments_page(page_num:usize){
    let mut page =comment::get_comments_page(page_num, PAGE_SIZE).await.unwrap_or_else(|e|{
        log::error!("{e}");
        Page::new(0, 0)
    });
    page.get_records_mut().iter_mut().for_each(|item|{
        todo!("未完成");
    });
}