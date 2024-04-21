/*
 * @Author: lurendie 549700459@qq.com
 * @Date: 2024-02-24 22:58:03
 * @LastEditors: lurendie
 * @LastEditTime: 2024-04-19 15:57:18
 * @FilePath: \zero_blog\src\config.rs
 */
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::fs;
//配置文件结构体
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub server: Server,
    pub mysql: MysqlCon, //Mysql链接
    pub redis: ReidsCon, //Redis
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ReidsCon {
    pub port: u16,    //端口
    pub host: String, //IP地址
    pub db: u16,
    pub username: String,
    pub password: String,
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MysqlCon {
    pub port: u16,    //端口
    pub host: String, //IP地址
    pub data_base: String,
    pub user_name: String,
    pub password: String,
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Server {
    pub port: u16,             //端口
    pub host: String,          //IP地址
    pub front_adderss: String, //前端页面地址
}
static CONFIG: OnceCell<Config> = OnceCell::new();

pub fn default() -> &'static Config {
    CONFIG.get_or_init(|| {
        let yaml_str =
            fs::read_to_string("./conf/config.yaml").expect("Failed to read config.yaml");
        let config: Config = serde_yaml::from_str(&yaml_str).expect("Failed to parse config.yaml");
        config
    })
}
