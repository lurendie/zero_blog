use rbs::{to_value, Value};

use crate::constant::redis_key_constants;
/*
 * @Author: lurendie 549700459@qq.com
 * @Date: 2024-02-24 22:58:03
 * @LastEditors: lurendie
 * @LastEditTime: 2024-04-21 00:15:16
 * @FilePath: \zero_blog\src\service\tag_service.rs
 */
use crate::dao::tag_dao::get_list;

use super::redis_service;

pub async fn get_tags() -> Vec<Value> {
    let mut result = vec![];
    //1.查询redis缓存
    let redis_cache =
        redis_service::get_value_vec(redis_key_constants::TAG_CLOUD_LIST.to_string()).await;
    if let Some(redis_cache) = redis_cache {
        let arr = match redis_cache {
            Value::Array(arr) => {
                log::error!(
                    "key:{} 数据存在",
                    redis_key_constants::TAG_CLOUD_LIST.to_string()
                );
                arr
            }
            _ => vec![],
        };
        return arr;
    }
    //2.查询数据库
    get_list()
        .await
        .unwrap_or_default()
        .iter()
        .for_each(|item| result.push(to_value!(item)));

    //保存到Redis
    redis_service::set_value_vec(
        redis_key_constants::TAG_CLOUD_LIST.to_string(),
        &to_value!(&result),
    )
    .await;
    result
}
