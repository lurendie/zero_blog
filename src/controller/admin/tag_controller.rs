use crate::{
    middleware::AppClaims,
    models::{vo::page_request::SearchRequest, Result},
    service::TagService,
};
use actix_jwt_session::Authenticated;
use actix_web::{routes, web, Responder};
use rbatis::IPageRequest;
use rbs::{to_value, value::map::ValueMap};

#[routes]
#[get("/tags")]
pub async fn get_all_tags(
    _: Authenticated<AppClaims>,
    params: web::Query<SearchRequest>,
) -> impl Responder {
    if params.get_page_num() <= 0 || params.get_page_size() <= 0 {
        return Result::error("参数有误!".to_string()).ok_json();
    }

    let tags_result =
        TagService::get_tags_by_page(params.get_page_num() as u64, params.get_page_size() as u64)
            .await;
    match tags_result {
        Ok(page_list) => {
            let mut map: ValueMap = ValueMap::new();

            map.insert(to_value!("pageNum"), to_value!(page_list.page_no()));
            map.insert(to_value!("pageSize"), to_value!(page_list.page_size()));
            map.insert(to_value!("pages"), to_value!(page_list.pages()));
            map.insert(to_value!("total"), to_value!(page_list.total()));
            map.insert(to_value!("list"), to_value!(page_list.records));

            Result::ok("请求成功！".to_string(), Some(to_value!(map))).ok_json()
        }
        Err(e) => Result::error(e.to_string()).ok_json(),
    }
}
