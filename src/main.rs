use actix_web::{web, App, HttpServer, Responder, HttpResponse};

//配置项
mod config;
//Rbatis
mod rbatis;

mod models;

async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Hello, Actix-web!")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let conf =config::default();

    HttpServer::new(|| {
        App::new()
            .app_data(rbatis::init_rbatis()) //初始化Rbatis
            .route("/", web::get().to(greet)) // 当访问根路径时调用greet函数
    })
        //.bind("127.0.0.1:8080")?
        .bind(format!("{}:{}",conf.server.address,conf.server.port))?
        .run()
        .await
}