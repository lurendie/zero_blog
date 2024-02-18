use actix_web::http::header::{AUTHORIZATION, ACCEPT, ACCESS_CONTROL_ALLOW_ORIGIN, CONTENT_TYPE, ACCESS_CONTROL_REQUEST_HEADERS};
use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use crate::config::Config;
use crate::controller::{index_controller,blog_controller};

pub async fn run(conf : &Config) ->std::io::Result<()>{
    HttpServer::new(|| {
    // 配置 CORS
    let cors = Cors::default()
    //.allowed_origin(&conf.server.front_adderss) // 只允许 example.com 域进行跨域请求,暂时不开启
    .allow_any_origin()
    .allowed_methods(vec!["GET", "POST","PUT","DELETE"]) // 允许的 HTTP 方法
    .allowed_headers(vec![AUTHORIZATION, ACCEPT,ACCESS_CONTROL_ALLOW_ORIGIN,CONTENT_TYPE,ACCESS_CONTROL_REQUEST_HEADERS])
    .max_age(3600); // 设置 preflight 缓存的最大时间
    App::new()
    .wrap(cors)//允许跨域请求
    .service(index_controller::site)
    .service(blog_controller::blogs)
        //.service(web::scope("/admin"))//
    .service(blog_controller::category)
    .service(blog_controller::blog)
    .service(blog_controller::tag)    
    .default_service(web::to(index_controller::default))
    })
    .bind(format!("{}:{}",conf.server.address,conf.server.port))?
    .run()
    .await
}



