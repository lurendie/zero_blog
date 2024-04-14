use crate::models::vo::result::Result;
use crate::service::about_service;
use actix_web::{get, Responder};

//关于我
#[get("/about")]
pub(crate) async fn about() -> impl Responder {
    let about = about_service::get_about().await;
    Result::ok("请求成功".to_string(), Some(about)).ok_json()
}
