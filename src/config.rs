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
    pub server: ServerConfig,
    pub mysql: MysqlConfig, //Mysql链接
    pub redis: ReidsConfig, //Redis
}
/**
 * Redis 连接信息结构体
 */
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ReidsConfig {
    pub port: u16,    //端口
    pub host: String, //IP地址
    pub db: u16,
    pub username: String,
    pub password: String,
    pub ttl: i64,
}
/**
 * MySQL 配置信息结构体
 */
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct MysqlConfig {
    pub port: u16,    //端口
    pub host: String, //IP地址
    pub data_base: String,
    pub user_name: String,
    pub password: String,
}
/**
 * Server 配置信息结构体
 */
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ServerConfig {
    pub port: u16,             //端口
    pub host: String,          //IP地址
    pub front_adderss: String, //前端页面地址
    pub token_expires: i64,    //token 过期时间
}
pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    let yaml_str = fs::read_to_string("./config/config.yaml").expect("Failed to read config.yaml");
    serde_yaml::from_str::<Config>(&yaml_str).expect("Failed to parse config.yaml")
});

/**
 * 获取配置信息
 */
pub fn _get_app_config() -> Config {
    CONFIG.clone()
}
