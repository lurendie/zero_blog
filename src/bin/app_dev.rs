//配置项
use nblog::config;
//Rbatis
pub use nblog::rbatis;
//模型
pub use nblog::models;
// 应用
pub use nblog::app;

//控制器
pub use nblog::controller;

//日志
use nblog::log4rs;
//服务
pub use nblog::service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //1. 初始化配置
    let conf = config::default();
    //2. 初始化日志
    log4rs::Log4rs::new();
    //3. Service run
    app::run(conf).await
}
