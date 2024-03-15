use actix_web::get;
use actix_web::{http::header, HttpResponse, Responder};
use crate::service::friend_service;
use crate::models::vo::result::Result;



//获取友链信息
#[get("/friends")]
pub(crate) async fn get_friend()->impl Responder{
    let friend =friend_service::get_friend().await;
    let result =Result::ok("请求成功".to_string(), Some(friend));
    HttpResponse::Ok().insert_header(header::ContentType(mime::APPLICATION_JSON)).json(result)
   
}