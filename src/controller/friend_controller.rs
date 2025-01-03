use crate::app_state::AppState;
use crate::models::vo::result::Result;
use crate::service::FriendService;
use actix_web::Responder;
use actix_web::{get, web, HttpResponse};

//获取友链信息
#[get("/friends")]
pub(crate) async fn get_friend(app: web::Data<AppState>) -> impl Responder {
    match FriendService::get_friend(app.get_mysql_pool()).await {
        Ok(friend) => HttpResponse::Ok().json(Result::ok("请求成功".to_string(), Some(friend))),
        Err(e) => Result::error(e.to_string()).ok_json(),
    }
}
