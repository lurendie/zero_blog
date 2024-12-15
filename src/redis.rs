use std::sync::LazyLock;

use crate::config::CONFIG;

pub static REDIS_URL: LazyLock<String> = LazyLock::new(|| {
    format!(
        "redis://{}:{}@{}:{}/{}",
        CONFIG.redis.username,
        CONFIG.redis.password,
        CONFIG.redis.host,
        CONFIG.redis.port,
        CONFIG.redis.db
    )
});

//Redis客户端
pub static REDIS_CL_IENT: LazyLock<deadpool_redis::Pool> = LazyLock::new(|| {
    let client = deadpool_redis::Config::from_url(REDIS_URL.as_str())
        .create_pool(Some(deadpool_redis::Runtime::Tokio1))
        .unwrap();
    // client.get_connection();
    // log::info!("redis连接初始化完成！");
    client
});

// pub async fn get_redis_client() -> deadpool_redis::Connection {
//     REDIS_CL_IENT
//         .timeout_get(&deadpool_redis::::::config::Timeouts::wait_millis(
//             5000,
//         ))
//         .await
//         .expect("redis连接超时！")
// }

// //redis 单元测试
// #[cfg(test)]
// mod test {
//     use super::REDIS;
//     use redis::{Cmd, ToRedisArgs};

//     #[test]
//     fn test_connect() -> redis::RedisResult<()> {
//         let mut con = REDIS.get_connection().unwrap();
//         let _ = Cmd::set("my_key".to_redis_args(), 1.to_redis_args()).query::<()>(&mut con);
//         Ok(())
//     }
// }
