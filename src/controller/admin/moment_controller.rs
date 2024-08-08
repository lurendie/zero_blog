use crate::middleware::AppClaims;
//创建动态
use crate::models::dto::moment_dto::MomentDTO;
use crate::models::{Result,vo::page_request::SearchRequest};
use crate::service::MomentService;
use actix_jwt_session::Authenticated;
use actix_web::{routes, web, Responder};
use rbatis::{IPage, IPageRequest};
use rbs::value::map::ValueMap;
use rbs::to_value;

/**
 * 创建动态
 */
#[routes]
#[post("/moment")]
pub async fn create_moment(moment: web::Json<MomentDTO>,_:Authenticated<AppClaims>) -> impl Responder {
    let moment = MomentService::create_moment(moment.into_inner()).await;
    match moment {
        Ok(_) => Result::ok_no_data("更新成功".to_string()).ok_json(),
        Err(e) => Result::error(e.to_string()).error_json(),
    }
}

#[routes]
#[get("/moments")]
pub async fn moments(_:Authenticated<AppClaims>, query:web::Query<SearchRequest>)-> impl Responder{
    //查询所有moments
    if query.0.get_page_num() ==0{
       return Result::error("参数有误！".to_string()).error_json();
    }
    //分页查询
   let page_list = MomentService::get_moments(query.0.get_page_num() as usize).await;
    let mut value_map = ValueMap::new();
    value_map.insert(to_value!("pageNum"), to_value!(page_list.page_no()));
    value_map.insert(to_value!("pageSize"), to_value!(page_list.page_size()));
    value_map.insert(to_value!("pages"), to_value!(page_list.pages()));
    value_map.insert(to_value!("total"), to_value!(page_list.total()));
    value_map.insert(to_value!("list"), to_value!(page_list.get_records()));
    Result::ok("请求成功".to_string(), Some(to_value!(value_map))).ok_json() // 返回一个包含map的JSON响应
    
}
