use std::collections::HashMap;

use crate::enums::DataBaseError;
use crate::redis;
use crate::CONFIG;
use deadpool_redis::redis::AsyncCommands;
use rbs::Value;

pub struct RedisService;

impl RedisService {
    /**
        根据KEY HashName 查询HashMap<String, Value>
    */
    pub async fn get_hash_key(
        key: String,
        hash: String,
    ) -> Result<HashMap<String, Value>, Box<dyn std::error::Error>> {
        //1.获取连接
        let mut connection = redis::get_connection().await?;
        //2.判断key是否存在
        if !connection
            .exists::<String, bool>(key.clone())
            .await
            .unwrap_or(false)
        {
            log::info!("redis KEY: {} 没有检索到数据 ", key);
            return Err(Box::new(DataBaseError::Unknown));
        }
        let redis_reuslt = connection
            .hget::<String, String, String>(key.to_owned(), hash.to_owned())
            .await?;
        Ok(serde_json::from_str::<HashMap<String, Value>>(
            redis_reuslt.as_str(),
        )?)
    }

    /**
     * 根据HashName key保存HashMap<String, Value>
     */
    pub async fn set_hash_key(key: String, hash: String, value: &HashMap<String, Value>) {
        //redis序列化
        let value_str = serde_json::to_string(&value).unwrap_or_default();
        match redis::get_connection().await {
            Ok(mut connection) => {
                let _ = connection
                    .hset::<String, String, String, String>(key.clone(), hash, value_str)
                    .await;
                RedisService::set_expire(key).await;
            }
            Err(e) => {
                log::error!("redis 设置key:{} 获取连接异常:{}", key, e);
            }
        }
    }
    /**
     * Set `key` `value`字符串
     */
    pub async fn set_value_map(key: String, value: &HashMap<String, Value>) {
        //1.序列化
        let value_str = serde_json::to_string(&value).unwrap_or_default();
        //2.获取连接
        match redis::get_connection().await {
            Ok(mut connection) => {
                let _ = connection
                    .set::<String, String, String>(key.clone(), value_str)
                    .await;
                RedisService::set_expire(key).await;
            }
            Err(e) => log::error!("redis 设置key:{} 获取连接异常:{}", key, e),
        }
    }

    /**
     * 获取`key`字符串
     */
    pub async fn get_value_map(
        key: String,
    ) -> Result<HashMap<String, Value>, Box<dyn std::error::Error>> {
        //1.获取连接
        match redis::get_connection().await {
            Ok(mut connection) => {
                let result = connection.get::<String, String>(key).await?;
                Ok(serde_json::from_str::<HashMap<String, Value>>(
                    result.as_str(),
                )?)
            }
            Err(e) => Err(Box::new(e)),
        }
    }

    /**
     * Set `key` `value`字符串
     */
    pub async fn set_value_vec(key: String, value: &Value) {
        //如果KEY或者VALUE为空则不设置
        if key.is_empty() || value.is_empty() {
            return;
        }
        //1.序列化
        let value_str = serde_json::to_string(value).unwrap_or_default();
        //判断转换字符串是否为空
        if value_str.is_empty() {
            log::error!("redis 设置key{}的value为空", key);
            return;
        }
        //2.获取连接
        match redis::get_connection().await {
            //3.获取连接成功
            Ok(mut con) => {
                //4.设置数据
                let _ = con
                    .set::<String, String, String>(key.clone(), value_str)
                    .await;
                //5.设置过期时间
                RedisService::set_expire(key).await;
            }
            //获取连接失败
            Err(e) => {
                log::error!("redis 设置key{}获取连接异常:{}", key, e);
            }
        }
    }

    /**
     * 获取`key`字符串
     */
    pub async fn get_value_vec(key: String) -> Option<Value> {
        //1.获取连接
        match redis::get_connection().await {
            //2.获取连接成功
            Ok(mut connection) => {
                //3.a.判断key是否存在
                if !connection
                    .exists::<String, bool>(key.clone())
                    .await
                    .unwrap_or(false)
                {
                    log::info!("redis KEY: {} 没有检索到数据 ", key);
                    return None;
                }
                //4.获取数据
                match connection.get::<String, String>(key.clone()).await {
                    Ok(result) => {
                        //redis 反序列化
                        Some(serde_json::from_str(result.as_str()).unwrap_or_default())
                    }
                    Err(e) => {
                        log::error!("redis {} 反序列化错误：{}", key, e);
                        None
                    }
                }
            }
            //获取连接失败
            Err(e) => {
                log::error!("redis 设置key: {} 获取连接异常:{}", key, e);
                None
            }
        }
    }

    /**
     * 设置key的过期时间
     */
    pub async fn set_expire(key: String) {
        //获取连接
        if let Ok(mut redis) = redis::get_connection().await {
            let _ = redis.expire::<String, i64>(key, CONFIG.redis.ttl).await;
        } else {
            log::error!("设置key: {} 的过期时间失败", key);
        }
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
