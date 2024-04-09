use crate::models::vo::{page_request::PageRequest, result::Result};
use crate::service::comments_service;
use actix_web::web::Query;
use actix_web::{get, Responder};
use rbatis::{IPage, IPageRequest};
use rbs::to_value;
use rbs::value::map::ValueMap;

#[get("/comments")]
pub(crate) async fn get_comments(data: Option<Query<PageRequest>>) -> impl Responder {
    //println!("page:{:?}", data);
    if data.is_none() {
        return Result::error("获取数据失败!".to_string()).error_json();
    }
    let mut comments = ValueMap::new();
    let page_request = data.unwrap();

    let list = comments_service::get_comments_page(
        page_request.page_num() as usize,
        page_request.page(),
        page_request.blog_id(),
    )
    .await;
    comments.insert("list".into(), to_value!(list.get_records()));
    comments.insert("totalPage".into(), rbs::Value::U64(list.pages()));
    let mut data = ValueMap::new();
    data.insert("comments".into(), to_value!(comments));
    data.insert(
        "allComment".into(),
        to_value!(rbs::Value::String(
            comments_service::get_all_comments(page_request.blog_id()).await
        )),
    );
    data.insert(
        "closeComment".into(),
        to_value!(comments_service::get_close_comments(page_request.blog_id()).await),
    );
    Result::ok("获取成功!".to_string(), Some(data)).ok_json()
}
