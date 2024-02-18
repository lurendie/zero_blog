use std::collections::HashMap;
use actix_web::{get, HttpResponse, Responder};
use actix_web::http::header;
use log::info;
use crate::service::{site_setting_service,category_service,blog_service,tag_service};
use rbs::to_value;
use rbs::Value;
use crate::models::vo::result::Result;

//返回数据
#[get("/site")]
pub async fn site() -> impl Responder {
        let mut map:HashMap<String, Value>=site_setting_service::get_site_info().await;
        let category_list =category_service::get_list().await;
        let random_list =blog_service::get_blog_list_random().await;
        let new_list =blog_service::get_blog_list_new().await;
        let tag_list=tag_service::get_tags().await;
        map.insert("newBlogList".to_string(),to_value!(new_list.unwrap()));
        map.insert("categoryList".to_string(),to_value!(category_list));
        map.insert("tagList".to_string(),to_value!(tag_list.unwrap())); 
        map.insert("randomBlogList".to_string(),to_value!(random_list.unwrap_or_default()));
        let result :Result<HashMap<String, Value>>=Result::new(200,String::from("请求成功！"),Some(map));
        HttpResponse::Ok().insert_header(header::ContentType(mime::APPLICATION_JSON)).json(result)
    }


pub async fn default() -> impl Responder {
    info!("404,找不到页面");
    HttpResponse::Found().content_type(mime::TEXT_HTML_UTF_8).body("404,找不到页面")
}