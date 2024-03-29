use std::collections::HashMap;

use actix_web::{get, HttpResponse, Responder};
use actix_web::http::header;
use rbs::value::map::ValueMap;
use rbs::{to_value, Value};
use crate::models::vo::result::Result;
use crate::service::blog_service;

#[get("/archives")]
pub(crate) async fn archives()->impl Responder {
    
    let mut data:HashMap<String, Value> = HashMap::new();
    //todo blogMap必须固定顺序
    let blog_map:ValueMap=blog_service::get_archives().await;
    let count =blog_service::get_archives_count().await;
    data.insert("blogMap".to_string(), to_value!(blog_map));
    data.insert("count".to_string(), to_value!(count));
    let result =Result::ok("请求成功".to_string(), Some(data));
    HttpResponse::Ok().insert_header(header::ContentType(mime::APPLICATION_JSON)).json(result)
}