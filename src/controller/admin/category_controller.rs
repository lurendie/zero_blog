use std::collections::HashMap;

use actix_jwt_session::Authenticated;
use actix_web::{routes, web, Responder};
use rbatis::{IPage, IPageRequest};
use rbs::{to_value, Value};
use crate::models::vo::page_request::SearchRequest;
use crate::models::vo::result::Result;
use crate::middleware::AppClaims;
use crate::service::CategoryService;




#[routes]
#[get("/categories")]
pub async fn categories(_:Authenticated<AppClaims>,params:web::Query<SearchRequest>)-> impl Responder{
    if params.get_page_num() <=0 || params.get_page_size()<=0{
       return  Result::error("参数有误!".to_string()).error_json();
    }
    match CategoryService::get_page_categories(params.get_page_num() as u64, params.get_page_size()as u64).await {
        Ok(data) => {
            let mut  map = HashMap::new();
            map.insert(to_value!("pageNum"), to_value!(data.page_no()));
            map.insert(to_value!("pageSize"), to_value!(data.page_size()));
            map.insert(to_value!("pages"), to_value!(data.pages()));
            map.insert(to_value!("total"), to_value!(data.total()));
            map.insert(to_value!("list"), to_value!(data.get_records()));
            Result::<Value>::ok("获取成功!".to_string(), Some(to_value!(map))).ok_json()
        }
        Err(e) => {
             Result::error(e.to_string()).error_json()
    }
    
    }
}