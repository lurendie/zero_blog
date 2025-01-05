/*
 * @Author: lurendie
 * @Date: 2024-02-24 22:58:03
 * @LastEditors: lurendie
 * @LastEditTime: 2024-05-17 12:18:04
 */
use serde::{Deserialize, Serialize};
use std::{fs, sync::LazyLock};
//配置文件结构体
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Config {
    server: ServerConfig,
    mysql: MysqlConfig, //Mysql链接
    redis: ReidsConfig, //Redis
}
/**
 * Redis 连接信息结构体
 */
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ReidsConfig {
    pub(crate) port: u16,    //端口
    pub(crate) host: String, //IP地址
    pub(crate) db: u16,
    pub(crate) username: String,
    pub(crate) password: String,
    pub(crate) ttl: i64,
}
/**
 * MySQL 配置信息结构体
 */
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct MysqlConfig {
    pub(crate) port: u16,    //端口
    pub(crate) host: String, //IP地址
    pub(crate) data_base: String,
    pub(crate) user_name: String,
    pub(crate) password: String,
}
/**
 * Server 配置信息结构体
 */
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ServerConfig {
    pub(crate) port: u16,             //端口
    pub(crate) host: String,          //IP地址
    pub(crate) front_adderss: String, //前端页面地址
    pub(crate) token_expires: i64,    //token 过期时间
}
pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    let yaml_str = fs::read_to_string("./config/config.yaml").expect("Failed to read config.yaml");
    serde_yaml::from_str::<Config>(&yaml_str).expect("Failed to parse config.yaml")
});

impl Config {
    pub fn get_mysql_config(&self) -> MysqlConfig {
        self.mysql.clone()
    }

    pub fn get_redis_config(&self) -> ReidsConfig {
        self.redis.clone()
    }

    pub fn get_server_config(&self) -> ServerConfig {
        self.server.clone()
    }
}

/**
 * 获取配置信息
 */
pub fn _get_app_config() -> Config {
    CONFIG.clone()
}
