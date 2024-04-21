/*
 * @Author: lurendie 549700459@qq.com
 * @Date: 2024-04-13 13:32:10
 * @LastEditors: lurendie
 * @LastEditTime: 2024-04-21 23:33:46
 * @FilePath: \zero_blog\src\service\redis_service.rs
 */
use std::collections::HashMap;

use crate::redis::REDIS;
use rbs::Value;
use redis::{Commands, ConnectionLike};

/**
    根据KEY HashName 查询HashMap<String, Value>
*/
pub async fn get_hash_key(key: String, hash: String) -> Option<HashMap<String, Value>> {
    //1.获取连接
    let mut con = REDIS.get_connection().unwrap();
    let redis_reuslt = con
        .hget::<String, String, String>(key.to_owned(), hash.to_owned())
        .unwrap_or_else(|_| {
            log::error!("hash:{} key:{} 缓存数据不存在", hash, key);
            String::new()
        });
    //redis 反序列化
    if redis_reuslt.is_empty() {
        //如果是null 则返回空集合
        return None;
    } else {
        return Some(
            serde_json::from_str::<HashMap<String, Value>>(redis_reuslt.as_str()).unwrap(),
        );
    }
}

/**
 * 根据HashName key保存HashMap<String, Value>
 */
pub async fn set_hash_key(key: String, hash: String, value: &HashMap<String, Value>) {
    //redis序列化
    let value_str = serde_json::to_string(&value).unwrap();
    let mut con: Box<dyn ConnectionLike> = Box::new(REDIS.get_connection().unwrap());
    redis::cmd("HSET")
        .arg(&[key, hash, value_str])
        .execute(con.as_mut())
}
/**
 * Set `key` `value`字符串
 */
pub async fn set_value_map(key: String, value: &HashMap<String, Value>) {
    //1.序列化
    let value_str = serde_json::to_string(&value).unwrap();
    //2.获取连接
    let mut con = REDIS.get_connection().unwrap();

    let _ = con.set::<String, String, String>(key, value_str);
}

/**
 * 获取`key`字符串
 */
pub async fn get_value_map(key: String) -> Option<HashMap<String, Value>> {
    //1.获取连接
    let mut con = REDIS.get_connection().unwrap();
    let result = con.get::<String, String>(key).unwrap_or_default();
    //redis 反序列化
    if result.is_empty() {
        return None;
    } else {
        return Some(serde_json::from_str::<HashMap<String, Value>>(result.as_str()).unwrap());
    }
}

/**
 * Set `key` `value`字符串
 */
pub async fn set_value_vec(key: String, value: &Value) {
    //1.序列化
    let value_str = serde_json::to_string(value).unwrap();
    //2.获取连接
    let mut con = REDIS.get_connection().unwrap();

    let _ = con.set::<String, String, String>(key, value_str);
}

/**
 * 获取`key`字符串
 */
pub async fn get_value_vec(key: String) -> Option<Value> {
    //1.获取连接
    let mut con = REDIS.get_connection().unwrap();
    let result = con.get::<String, String>(key).unwrap_or_default();
    //redis 反序列化
    if result.is_empty() {
        return None;
    } else {
        return Some(serde_json::from_str(result.as_str()).unwrap());
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_json_get() {
        let mut map: HashMap<String, Value> = HashMap::new();
        map.insert("1".to_string(), Value::String("value1".to_string()));

        //let _ = super::set_value("my_sql".to_string(), &map);
    }
}
