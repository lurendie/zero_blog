use std::collections::HashMap;
use actix_web::{get, HttpResponse, Responder};
use actix_web::http::header;
use actix_web::web::Query;
use rbs::Value;
use crate::service;
use crate::models::vo::result::Result;
use service::blog_service;

//按置顶、创建时间排序 分页查询博客简要信息列表
#[get("/blogs")]
pub async  fn blogs(params: Query<HashMap<String, usize>>) ->impl Responder{
    //提供默认值page_num.expect("异常！")
    let page :usize;
    if params.get("pageNum") !=None{
        page= *params.get("pageNum").expect("异常");
    }else {
        page=1;
    }
    let  page=blog_service::get_blog_list_by_is_published(Some(page as u64)).await;
    let result : Result<HashMap<String, Value>> = Result::<HashMap<String, Value>>::ok(String::from("请求成功！"), Some(page));
    HttpResponse::Ok().insert_header(header::ContentType(mime::APPLICATION_JSON)).json(result)
}

#[get("/blog")]
pub async  fn blog(params: Query<HashMap<String, String>>) ->impl Responder{
    //提供默认值page_num.expect("异常！")
    let id :u16;
    if params.get("id") !=None{
        id= params.get("id").expect("异常").parse::<u16>().expect("转换异常");
    }else {
        id=0;
    }
    let blog =blog_service::get_by_id(id).await;
    println!("{:?}",blog);
    let result =Result::new(200,"请求成功".to_string(), blog);
    HttpResponse::Ok().insert_header(header::ContentType(mime::APPLICATION_JSON)).json(result)
}

#[get("/category")]
pub async  fn category(params: Query<HashMap<String, String>>) ->impl Responder{
    //提供默认值page_num.expect("异常！")
    let category_name :String;
    if params.get("categoryName") !=None{
        category_name= params.get("categoryName").expect("异常").clone();
    }else {
        category_name=String::new();
    }
    let page :usize;
    if params.get("pageNum") !=None{
        page= params.get("pageNum").expect("转换失败").parse::<u32>().expect("转换失败")as usize;
    }else {
        page=1;
    }
    let page=blog_service::get_by_name(category_name,page).await;
    let result : Result::<HashMap<String,Value>> = Result::<HashMap<String,Value>>::ok(String::from("请求成功！"),Some(page));
    HttpResponse::Ok().insert_header(header::ContentType(mime::APPLICATION_JSON)).json(result)
}

#[get("/tag")]
pub async  fn tag(params: Query<HashMap<String, String>>) ->impl Responder{
    //提供默认值page_num.expect("异常！")
    let tag_name :String;
    if params.get("tagName") !=None{
        tag_name= params.get("tagName").expect("异常").clone();
    }else {
        tag_name=String::new();
    }
    let page :usize;
    if params.get("pageNum") !=None{
        page= params.get("pageNum").expect("转换失败").parse::<u32>().expect("转换失败")as usize;
    }else {
        page=1;
    }
    let page=blog_service::get_by_tag_name(tag_name,page).await;
    let result : Result::<HashMap<String,Value>> = Result::<HashMap<String,Value>>::ok(String::from("请求成功！"),Some(page));
    HttpResponse::Ok().insert_header(header::ContentType(mime::APPLICATION_JSON)).json(result)
}



