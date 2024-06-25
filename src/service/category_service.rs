/*
 * @Author: lurendie
 * @Date: 2024-02-24 22:58:03
 * @LastEditors: lurendie
 * @LastEditTime: 2024-04-19 23:46:51
 * @FilePath: \zero_blog\src\service\category_service.rs
 */

use rbs::value::map::ValueMap;
use rbs::{to_value, Value};

use crate::constant::redis_key_constants;
use crate::dao::{
    blog_dao,
    category_dao::{self, get_list as getList},
};
use crate::models::vo::categorie::Categorie;
use crate::models::vo::serise::Series;
use crate::service::redis_service;
/**
 * 查询所有分类
 */
pub async fn get_list() -> Vec<Value> {
    //1.查询Redis
    let result =
        redis_service::get_value_vec(redis_key_constants::CATEGORY_NAME_LIST.to_string()).await;
    if let Some(result) = result {
        let arr = match result {
            Value::Array(arr) => {
                log::error!(
                    "key:{} 数据存在",
                    redis_key_constants::CATEGORY_NAME_LIST.to_string()
                );
                arr
            }
            _ => vec![],
        };
        return arr;
    }
    //2.查询数据库
    let mut result = vec![];
    getList()
        .await
        .iter()
        .for_each(|item| result.push(to_value!(item)));

    //3.保存Redis
    log::info!(
        "key:{} 数据不存在",
        redis_key_constants::CATEGORY_NAME_LIST.to_string()
    );
    redis_service::set_value_vec(
        redis_key_constants::CATEGORY_NAME_LIST.to_string(),
        &to_value!(&result),
    )
    .await;
    result
}

/**
 * 查询分类名称
 */
pub async fn get_categorys_count() -> ValueMap {
    let mut map = ValueMap::new();
    let mut legend = vec![];
    let mut series = vec![];

    for item in category_dao::get_list().await {
        legend.push(to_value!(item.name.clone()));
        let series_item = Series::new(
            item.id,
            item.name.clone(),
            blog_dao::get_category_count(item.name).await,
        );
        series.push(series_item);
    }
    map.insert(to_value!("legend"), to_value!(legend));
    map.insert(to_value!("series"), to_value!(series));
    map
}

pub(crate) async fn get_categories() -> Vec<Categorie> {
    let mut list = vec![];
    category_dao::get_list().await.iter().for_each(|item| {
        list.push(Categorie::new(
            Some(item.id.clone()),
            item.name.clone(),
            vec![],
        ))
    });
    list
}
