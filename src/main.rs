
//配置项
pub mod config;
//Rbatis
pub mod rbatis;
pub mod models;
mod log4rs;
pub mod app;
pub mod controller;
pub mod service;
mod dao;
mod constant;
mod r#enum;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //初始化配置
    let conf =config::default();
    //初始化Rabits
    rbatis::init_rbatis(conf).await.expect("数据库初始化失败");
    //初始化日志
    log4rs::Log4rs::new();
    //Service run
    app::run(conf).await
}



