/*
 * @Author: lurendie
 * @Date: 2024-02-24 22:58:03
 * @LastEditors: lurendie
 * @LastEditTime: 2024-04-21 00:29:02
 * @FilePath: \zero_blog\src\rbatis.rs
 */
use std::sync::{Arc, LazyLock};

use crate::config::CONFIG;
use log::LevelFilter;
use rbatis::{
    executor::RBatisConnExecutor, intercept_log::LogInterceptor, rbatis::RBatis, DefaultPool,
};
use rbdc_mysql::{options::MySqlConnectOptions, Driver};

// 定义一个静态变量来存储Rbatis连接实例。
// 定义一个全局的 RBatis 实例，使用 Lazy 来保证线程安全的单次初始化
pub static RBATIS: LazyLock<RBatis> = LazyLock::new(|| {
    let mut rbatis = RBatis::new();
    //修改默认日志INFO等级为 DEBUG
    let opts = MySqlConnectOptions::new()
        .username(CONFIG.mysql.user_name.as_str())
        .password(CONFIG.mysql.password.as_str())
        .database(CONFIG.mysql.data_base.as_str())
        .host(CONFIG.mysql.host.as_str());
    let _ = &rbatis.init_option::<Driver, MySqlConnectOptions, DefaultPool>(Driver {}, opts);
    //log::info!("MySQL连接初始化完成！");
    rbatis.set_intercepts(vec![Arc::new(LogInterceptor::new(LevelFilter::Debug))]);
    rbatis
});

// 获取数据库连接
pub async fn get_conn() -> RBatisConnExecutor {
    match RBATIS.acquire().await {
        Ok(conn) => conn,
        Err(e) => panic!("获取数据库连接失败！err: {}", e),
    }
}
