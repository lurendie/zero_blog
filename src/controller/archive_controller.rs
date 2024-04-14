use crate::models::vo::result::Result;
use crate::service::blog_service;
use actix_web::{get, Responder};
use rbs::to_value;
use rbs::value::map::ValueMap;

#[get("/archives")]
pub(crate) async fn archives() -> impl Responder {
    let mut data = ValueMap::new();
    //todo blogMap必须固定顺序
    let blog_map = blog_service::get_archives().await;
    let count = blog_service::get_archives_count().await;
    data.insert(to_value!("blogMap"), to_value!(blog_map));
    data.insert(to_value!("count"), to_value!(count));
    Result::ok("请求成功".to_string(), Some(data)).ok_json()
}
