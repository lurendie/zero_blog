use std::sync::Arc;

use crate::config::Config;
use log::LevelFilter;
use once_cell::sync::Lazy;
use rbatis::{intercept_log::LogInterceptor, rbatis::RBatis, DefaultPool};
use rbdc_mysql::{options::MySqlConnectOptions, Driver};

// 定义一个静态变量来存储Rbatis连接实例。
// 定义一个全局的 RBatis 实例，使用 Lazy 来保证线程安全的单次初始化
pub static RBATIS: Lazy<RBatis> = Lazy::new(|| {
    let mut rbatis = RBatis::new();
    //修改默认日志INFO等级为 DEBUG
    rbatis.set_intercepts(vec![Arc::new(LogInterceptor::new(LevelFilter::Debug))]);
    rbatis
});

pub async fn init_rbatis(conf: &Config) -> Result<(), rbatis::Error> {
    // let db_url = format!(
    //     "mysql://{}:{}@{}/{}",
    //     conf.mysql.user_name, conf.mysql.password, conf.mysql.address, conf.mysql.data_base
    // );
    // RBATIS
    //     .link(Driver {}, &db_url)
    //     .await
    //     .expect("rbatis:连接数据库失败！！！");
    let opts = MySqlConnectOptions::new()
        .username(conf.mysql.user_name.as_str())
        .password(conf.mysql.password.as_str())
        .database(conf.mysql.data_base.as_str())
        .host(conf.mysql.address.as_str());
    let _ = RBATIS.init_option::<Driver, MySqlConnectOptions, DefaultPool>(Driver {}, opts);
    log::debug!("数据库连接初始化完成！");
    Ok(())
}
