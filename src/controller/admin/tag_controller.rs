use crate::{
    app_state::AppState,
    middleware::AppClaims,
    models::{vo::page_request::SearchRequest, Result},
    service::TagService,
};
use actix_jwt_session::Authenticated;
use actix_web::{routes, web, Responder};
use rbs::to_value;

#[routes]
#[get("/tags")]
pub async fn get_all_tags(
    _: Authenticated<AppClaims>,
    params: web::Query<SearchRequest>,
    app: web::Data<AppState>,
) -> impl Responder {
    if params.get_page_num() <= 0 || params.get_page_size() <= 0 {
        return Result::error("参数有误!".to_string()).ok_json();
    }

    let tags_result =
        TagService::get_tags_by_page(params.get_page_num(), params.get_page_size(),app.get_mysql_pool())
            .await;
    match tags_result {
        Ok(value_map) => Result::ok("请求成功！".to_string(), Some(to_value!(value_map))).ok_json(),
        Err(e) => Result::error(e.to_string()).ok_json(),
    }
}
