use actix_web::{get, http::header, HttpResponse, Responder};
use crate::service::about_service;
use crate::models::vo::result::Result;


//关于我
#[get("/about")]
pub(crate) async fn about()->impl Responder {
    let about= about_service::get_about().await;
    let result =Result::ok("请求成功".to_string(), Some(about));
    HttpResponse::Ok().insert_header(header::ContentType(mime::APPLICATION_JSON)).json(result)
}