use crate::models::vo::page_request::PageRequest;
use crate::models::vo::result::Result;
use crate::service;
use actix_web::http::header;
use actix_web::web::{Json, Query};
use actix_web::{routes, HttpResponse, Responder};
use rbs::{to_value, Value};
use service::blog_service;
use std::collections::HashMap;

//按置顶、创建时间排序 分页查询博客简要信息列表
#[routes]
#[options("/site")]
#[get("/blogs")]
pub async fn blogs(params: Query<PageRequest>) -> impl Responder {
    //提供默认值page_num.expect("异常！")
    let page =
        blog_service::get_blog_list_by_is_published(Some(params.get_page_num() as u64)).await;
    let result: Result<HashMap<String, Value>> =
        Result::<HashMap<String, Value>>::ok(String::from("请求成功！"), Some(page));
    HttpResponse::Ok()
        .insert_header(header::ContentType(mime::APPLICATION_JSON))
        .json(result)
}
#[routes]
#[get("/blog")]
pub async fn blog(params: Query<HashMap<String, String>>) -> impl Responder {
    //提供默认值page_num.expect("异常！")
    let id: u16;
    if params.get("id") != None {
        id = params
            .get("id")
            .expect("异常")
            .parse::<u16>()
            .expect("转换异常");
    } else {
        id = 0;
    }
    let blog = blog_service::get_by_id(id).await;
    let result = Result::new(200, "请求成功".to_string(), blog);
    HttpResponse::Ok()
        .insert_header(header::ContentType(mime::APPLICATION_JSON))
        .json(result)
}
#[routes]
#[get("/category")]
pub async fn category(params: Query<HashMap<String, String>>) -> impl Responder {
    //提供默认值page_num.expect("异常！")
    let category_name: String;
    if params.get("categoryName") != None {
        category_name = params.get("categoryName").expect("异常").clone();
    } else {
        category_name = String::new();
    }
    let page: usize;
    if params.get("pageNum") != None {
        page = params
            .get("pageNum")
            .expect("转换失败")
            .parse::<u32>()
            .expect("转换失败") as usize;
    } else {
        page = 1;
    }
    let page = blog_service::get_by_name(category_name, page).await;
    let result: Result<HashMap<String, Value>> =
        Result::<HashMap<String, Value>>::ok(String::from("请求成功！"), Some(page));
    HttpResponse::Ok()
        .insert_header(header::ContentType(mime::APPLICATION_JSON))
        .json(result)
}
#[routes]
#[get("/tag")]
pub async fn tag(params: Query<HashMap<String, String>>) -> impl Responder {
    //提供默认值page_num.expect("异常！")
    let tag_name: String;
    if params.get("tagName") != None {
        tag_name = params.get("tagName").expect("异常").clone();
    } else {
        tag_name = String::new();
    }
    let page: usize;
    if params.get("pageNum") != None {
        page = params
            .get("pageNum")
            .expect("转换失败")
            .parse::<u32>()
            .expect("转换失败") as usize;
    } else {
        page = 1;
    }
    let page = blog_service::get_by_tag_name(tag_name, page).await;
    let result: Result<HashMap<String, Value>> =
        Result::<HashMap<String, Value>>::ok(String::from("请求成功！"), Some(page));
    HttpResponse::Ok()
        .insert_header(header::ContentType(mime::APPLICATION_JSON))
        .json(result)
}

/**
 * 检测Blog PassWrod 的正确性
 */
#[routes]
#[post("/checkBlogPassword")]
pub async fn check_blog_password(data: Json<PageRequest>) -> impl Responder {
    if data.get_blog_id() > 0 {
        let blog_info = blog_service::get_by_id(data.get_blog_id()).await;
        if let Some(blog_info) = &blog_info {
            if let Some(password) = &blog_info.password {
                if *password == data.get_password() {
                    Result::ok("验证成功,密码正确!".to_string(), Some(to_value!(blog_info)))
                        .ok_json()
                } else {
                    Result::error("验证失败,密码错误!".to_string()).error_json()
                }
            } else {
                Result::error("验证失败,密码错误!".to_string()).error_json()
            }
        } else {
            Result::error("验证失败,密码错误!".to_string()).error_json()
        }
    } else {
        Result::error("参数有误!".to_string()).error_json()
    }
}
