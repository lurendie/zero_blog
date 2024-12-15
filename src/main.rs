//配置项
use zero_blog::{app::AppServer, log4rs};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //1. 初始化日志
    log4rs::Log4rs::new();
    //2. Service run
    AppServer::run().await
}
