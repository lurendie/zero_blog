use actix_jwt_session::Authenticated;
use actix_web::{
    routes,
    web::{self, Data},
    Responder,
};
use rbs::to_value;

use crate::{
    app_state::AppState,
    middleware::AppClaims,
    model::{ResponseResult, SearchRequest},
    service::CommentService,
};

#[routes]
#[get("/comments")]
pub async fn find_comments(
    _: Authenticated<AppClaims>,
    app: Data<AppState>,
    query: web::Query<SearchRequest>,
) -> impl Responder {
    let page_num = query.get_page_num();
    match CommentService::find_comments(page_num, app.get_mysql_pool()).await {
        Ok(comments) => {
            ResponseResult::ok("请求成功！".to_string(), Some(to_value!(comments))).json()
        }
        Err(e) => ResponseResult::error(e.to_string()).json(),
    }
}
