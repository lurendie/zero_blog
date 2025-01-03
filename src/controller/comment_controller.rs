use crate::app_state::AppState;
/*
 * @Author: lurendie
 * @Date: 2024-02-24 22:58:03
 * @LastEditors: lurendie
 * @LastEditTime: 2024-04-21 23:26:00
 */
use crate::models::vo::{page_request::SearchRequest, result::Result};
use crate::service::CommentService;
use actix_web::web::{self, Query};
use actix_web::{get, Responder};
use rbs::to_value;
use rbs::value::map::ValueMap;

#[get("/comments")]
pub(crate) async fn get_comments(
    data: Option<Query<SearchRequest>>,
    app: web::Data<AppState>,
) -> impl Responder {
    //println!("page:{:?}", data);
    if data.is_none() {
        return Result::error("获取数据失败!".to_string()).ok_json();
    }

    let page_request = match data {
        Some(page_request) => page_request,
        None => return Result::error("获取数据失败!".to_string()).ok_json(),
    };
    let connect = app.get_mysql_pool();
    let list = match CommentService::find_comments_page(
        page_request.get_page_num(),
        page_request.get_blog_id(),
        connect,
    )
    .await
    {
        Ok(list) => list,
        Err(e) => {
            return Result::error(e.to_string()).ok_json();
        }
    };

    let mut data = ValueMap::new();
    data.insert("comments".into(), to_value!(list));

    match CommentService::get_all_comments(page_request.get_blog_id(), connect).await {
        Ok(close_comment) => {
            data.insert("allComment".into(), to_value!(close_comment));
        }
        Err(e) => {
            return Result::error(e.to_string()).ok_json();
        }
    }
    match CommentService::get_close_comments(page_request.get_blog_id(), connect).await {
        Ok(close_comment) => {
            data.insert("closeComment".into(), to_value!(close_comment));
        }
        Err(e) => {
            return Result::error(e.to_string()).ok_json();
        }
    }

    Result::ok("获取成功!".to_string(), Some(to_value!(data))).ok_json()
}
