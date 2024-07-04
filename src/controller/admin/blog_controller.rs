use crate::service::{BlogService, CategoryService};
use crate::{
    middleware::AppClaims,
    models::{vo::blog_visibility::BlogVisibility, vo::page_request::SearchRequest, Result},
};
use actix_jwt_session::Authenticated;
use actix_web::web::Json;
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

/**
 * 博文可见性 置顶 密码 推荐
 */
#[routes]
#[put("/blog/{blog_id}/visibility")]
pub async fn visibility(
    path: web::Path<u16>,
    mut query: Json<BlogVisibility>,
    _: Authenticated<AppClaims>,
) -> impl Responder {
    let id = path.into_inner();
    query.set_id(id as u32);

    let row = blog_service::update_visibility(&query).await;
    if row {
        Result::ok_no_data("更新成功".to_string()).ok_json()
    } else {
        Result::error("更新失败".to_string()).error_json()
    }
}

#[routes]
#[put("/blog/top")]
pub async fn top(query: web::Query<BlogVisibility>, _: Authenticated<AppClaims>) -> impl Responder {
    let row = blog_service::update_visibility(&query).await;
    if row {
        Result::ok_no_data("更新成功".to_string()).ok_json()
    } else {
        Result::error("更新失败".to_string()).error_json()
    }
}

#[routes]
#[put("/blog/recommend")]
pub async fn recommend(
    query: web::Query<BlogVisibility>,
    _: Authenticated<AppClaims>,
) -> impl Responder {
    let row = blog_service::update_visibility(&query).await;
    if row {
        Result::ok_no_data("更新成功".to_string()).ok_json()
    } else {
        Result::error("更新失败".to_string()).error_json()
    }
    let _ = BlogService::update_by_id(id).await;
    Result::ok("请求成功".to_string(), Some(to_value!(false))).ok_json()
}
