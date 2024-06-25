use crate::service::{blog_service, category_service};
use crate::{
    middleware::AppClaims,
    models::{vo::page_request::SearchRequest, Result},
};
use actix_jwt_session::Authenticated;
use actix_web::{routes, web::Query, Responder};
use rbs::to_value;
use rbs::value::map::ValueMap;

#[routes]
#[get("/blogs")]
pub async fn blogs(query: Query<SearchRequest>, _: Authenticated<AppClaims>) -> impl Responder {
    let mut map = ValueMap::new();
    let page = blog_service::get_blog_all_page(&query.0).await;
    let categories = category_service::get_categories().await;
    map.insert(to_value!("blogs"), to_value!(page));
    map.insert(to_value!("categories"), to_value!(categories));
    Result::ok("请求成功".to_string(), Some(to_value!(map))).ok_json()
}
