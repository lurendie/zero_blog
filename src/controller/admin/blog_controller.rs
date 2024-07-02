use crate::service::{BlogService, CategoryService};
use crate::{
    middleware::AppClaims,
    models::{vo::page_request::SearchRequest, Result},
};
use actix_jwt_session::Authenticated;
use actix_web::{
    routes,
    web::{self, Query},
    Responder,
};
use rbs::to_value;
use rbs::value::map::ValueMap;

#[routes]
#[get("/blogs")]
pub async fn blogs(query: Query<SearchRequest>, _: Authenticated<AppClaims>) -> impl Responder {
    let mut map = ValueMap::new();
    let page = BlogService::get_blog_all_page(&query.0).await;
    let categories = CategoryService::get_categories().await;
    map.insert(to_value!("blogs"), to_value!(page));
    map.insert(to_value!("categories"), to_value!(categories));
    Result::ok("请求成功".to_string(), Some(to_value!(map))).ok_json()
}

#[routes]
#[put("/blog/{blog_id}/visibility")]
pub async fn visibility(
    path: web::Path<u16>,
    _query: Query<SearchRequest>,
    _: Authenticated<AppClaims>,
) -> impl Responder {
    let id = path.into_inner();
    let _ = BlogService::update_by_id(id).await;
    Result::ok("请求成功".to_string(), Some(to_value!(false))).ok_json()
}
