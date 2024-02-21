use std::collections::HashMap;

use actix_web::{get, HttpResponse, Responder};
use actix_web::http::header;
use actix_web::web::Query;
use rbs::{to_value, Value};
use crate::models::vo::result::Result;

#[get("/archives")]
pub(crate) async fn archives()->impl Responder {
    //todo!("未完成！！！");
    let mut data:HashMap<String, Value> = HashMap::new();
    let blog_map:HashMap<String, Value>=HashMap::new();
    data.insert("blogMap".to_string(), to_value!(blog_map));
    data.insert("count".to_string(), to_value!(0));
    let result =Result::ok("请求成功".to_string(), Some(data));
    HttpResponse::Ok().insert_header(header::ContentType(mime::APPLICATION_JSON)).json(result)
}