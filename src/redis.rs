use std::sync::LazyLock;

use redis::Client;

use crate::config::CONFIG;

//Redis客户端
pub static REDIS: LazyLock<Client> = LazyLock::new(|| {
    let client = redis::Client::open(format!(
        "redis://{}:{}@{}:{}/{}",
        CONFIG.redis.username,
        CONFIG.redis.password,
        CONFIG.redis.host,
        CONFIG.redis.port,
        CONFIG.redis.db
    ))
    .unwrap();
    // client.get_connection();
    // log::info!("redis连接初始化完成！");
    client
});

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
