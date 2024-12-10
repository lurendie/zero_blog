use std::collections::HashMap;

use actix_web::routes;
use actix_web::web::Path;
use actix_web::{web::Query, HttpResponse, Responder};
use rbatis::IPageRequest;
use rbs::to_value;
use rbs::value::map::ValueMap;
use crate::service::MomentService;

use crate::models::vo::result::Result;



//动态
#[routes]
#[get("/moments")]
pub(crate) async fn moments(params: Query<HashMap<String, String>>) -> impl Responder{
    let page :usize;
    if params.get("pageNum") !=None{
        page= params.get("pageNum").expect("转换失败").parse::<u32>().expect("转换失败")as usize;
    }else {
        page=1;
    }
    let page =MomentService::get_public_moments(page).await;
    let mut data:ValueMap=ValueMap::new();
    data.insert(to_value!("list"), to_value!(&page.records));
    data.insert(to_value!("totalPage"), to_value!(&page.pages()));
    let result = Result::ok("请求成功".to_string(), Some(data));
    HttpResponse::Ok().json(result)
}

#[routes]
#[post("/moment/like/{id}")]
pub async fn moment_like(id: Path<u16>) -> impl Responder {
    if *id <= 0 {
        return Result::error("参数有误!".to_string()).error_json();
    }
    let result = MomentService::moment_like(*id).await;
    dbg!(&result);
    if let Ok(row) = result {
        if row > 0 {
            return Result::ok("点赞成功".to_string(), Some(to_value!(row))).ok_json();
        } else {
            return Result::error("点赞失败".to_string()).error_json();
        }
    }
    Result::error("参数有误!".to_string()).error_json()
}