use crate::service::blog_service;
use crate::{
    middleware::AppClaims,
    models::{vo::page_request::SearchRequest, Result},
};
use actix_jwt_session::Authenticated;
use actix_web::{routes, web::Query, Responder};

#[routes]
#[get("/blogs")]
pub async fn blogs(query: Query<SearchRequest>, _: Authenticated<AppClaims>) -> impl Responder {
    let page = blog_service::get_blog_all_page(&query.0).await;

    Result::error("请求成功".to_string()).error_json()
}
