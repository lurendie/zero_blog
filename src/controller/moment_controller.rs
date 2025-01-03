use crate::app_state::AppState;
use crate::models::vo::page_request::SearchRequest;
use crate::service::MomentService;
use actix_web::web::Path;
use actix_web::{routes, web};
use actix_web::{web::Query, Responder};
use rbs::to_value;

use crate::models::vo::result::Result;

//动态
#[routes]
#[get("/moments")]
pub(crate) async fn moments(
    mut query: Query<SearchRequest>,
    app: web::Data<AppState>,
) -> impl Responder {
    //查询所有moments
    if query.0.get_page_num() == 0 {
        return Result::error("参数有误！".to_string()).ok_json();
    }
    query.0.set_page_size(Some(5));
    match MomentService::get_public_moments(
        query.0.get_page_num(),
        query.0.get_page_size(),
        app.get_mysql_pool(),
    )
    .await
    {
        Ok(data) => Result::ok("请求成功".to_string(), Some(to_value!(data))).ok_json(),
        Err(e) => Result::error(e.to_string()).ok_json(),
    }
}

#[routes]
#[post("/moment/like/{id}")]
pub async fn moment_like(id: Path<i64>, app: web::Data<AppState>) -> impl Responder {
    let id =id.into_inner();
    if id <= 0 {
        return Result::error("参数有误!".to_string()).ok_json();
    }
    let result = MomentService::moment_like(id, app.get_mysql_pool()).await;

    if let Ok(row) = result {
        return Result::ok("点赞成功".to_string(), Some(to_value!(row))).ok_json();
    }
    Result::error("参数有误!".to_string()).ok_json()
}
