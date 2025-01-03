use std::collections::HashMap;

use crate::app_state::AppState;
use crate::models::category::Category;
use crate::models::vo::page_request::SearchRequest;
use crate::models::vo::result::Result;
use crate::service::CategoryService;
use crate::{middleware::AppClaims, service::BlogService};
use actix_jwt_session::Authenticated;
use actix_web::{routes, web, Responder};
use rbs::{to_value, Value};

/**
 * 获取分类列表
 */
#[routes]
#[get("/categories")]
pub async fn categories(
    _: Authenticated<AppClaims>,
    params: web::Query<SearchRequest>,
    app: web::Data<AppState>,
) -> impl Responder {
    if params.get_page_num() <= 0 || params.get_page_size() <= 0 {
        return Result::error("参数有误!".to_string()).ok_json();
    }
    match CategoryService::get_page_categories(
        params.get_page_num() as u64,
        params.get_page_size() as u64,
        app.get_mysql_pool(),
    )
    .await
    {
        Ok(data) => {
            Result::<Value>::ok("获取成功!".to_string(), Some(to_value!(data))).ok_json()
        }
        Err(e) => Result::error(e.to_string()).ok_json(),
    }
}

/**
 * 修改分类
 */
#[routes]
#[put("/category")]
pub async fn update_category(
    _: Authenticated<AppClaims>,
    form: web::Json<Category>,
    app: web::Data<AppState>,
) -> impl Responder {
    //参数校验
    if form.get_name().is_empty() {
        return Result::error("参数有误!".to_string()).ok_json();
    }
    match form.get_id() == 0 {
        //新增分类
        true => {
            let _ =
                CategoryService::insert_category(form.get_name().to_string(), app.get_mysql_pool())
                    .await;
            return Result::ok_no_data("新增分类成功!".to_string()).ok_json();
        }
        //修改分类
        false => {
            let _ = CategoryService::update_category(form.0, app.get_mysql_pool()).await;
            return Result::ok_no_data("修改分类成功!".to_string()).ok_json();
        }
    }
}

/**
 * 删除分类
 */
#[routes]
#[delete("/category")]
pub async fn delete_category(
    _: Authenticated<AppClaims>,
    query: web::Query<HashMap<String, i64>>,
    app: web::Data<AppState>,
) -> impl Responder {
    let id = match query.get("id") {
        Some(id) => {
            if *id == 0 {
                return Result::error("参数有误!".to_string()).ok_json();
            }
            *id
        }
        None => return Result::error("参数有误!".to_string()).ok_json(),
    };
    // 查询分类下是否有文章
    let connection = app.get_mysql_pool();
    match BlogService::check_category_exist_blog(id, connection).await {
        Ok(true) => return Result::error("分类下存在文章,不能删除!".to_string()).ok_json(),
        Ok(false) => {
            // 删除分类
            match CategoryService::delete_category(id, connection).await {
                Ok(_) => Result::ok_no_data("删除分类成功!".to_string()).ok_json(),
                Err(e) => Result::ok_no_data(e.to_string()).ok_json(),
            }
        }
        Err(e) => Result::error(e.to_string()).ok_json(),
    }
}
