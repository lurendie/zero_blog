use std::str::FromStr;

/*
 * @Author: lurendie 549700459@qq.com
 * @Date: 2024-03-26 00:08:12
 * @LastEditors: lurendie
 * @LastEditTime: 2024-05-03 00:29:53
 * @FilePath: \zero_blog\src\app.rs
 */
use crate::config::Config;
use crate::controller::admin::index_controller as adminIndexController;
use crate::controller::{
    about_controller, archive_controller, blog_controller, comment_controller, friend_controller,
    index_controller, moment_controller,
};
use crate::middleware::VisiLog;
use crate::middleware::{create, Claims};
use actix_cors::Cors;
use actix_jwt_session::{deadpool_redis, Duration, JwtTtl, RefreshTtl};
use actix_web::http::header::{
    ACCEPT, ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_ORIGIN,
    ACCESS_CONTROL_EXPOSE_HEADERS, ACCESS_CONTROL_MAX_AGE, ACCESS_CONTROL_REQUEST_HEADERS,
    AUTHORIZATION, CONTENT_TYPE,
};
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{
    http::header::{HeaderName, HeaderValue},
    web, App, HttpServer,
};
/**
 * 服务启动
 * return io
 */
pub async fn run(conf: &'static Config) -> std::io::Result<()> {
    //创建JWT
    let redis = deadpool_redis::Config::from_url("redis://localhost:6379")
        .create_pool(Some(deadpool_redis::Runtime::Tokio1))
        .unwrap();
    let (storage, factory) = create::<Claims>(&redis);
    let jwt_ttl = JwtTtl(Duration::days(14));
    let refresh_ttl = RefreshTtl(Duration::days(3 * 31));
    HttpServer::new(move || {
        // 配置 CORS
        let cors = Cors::default()
            .allowed_origin(conf.server.front_adderss.as_str()) // 只允许 example.com 域进行跨域请求,暂时不开启
            //.allow_any_origin() //允许所有域访问
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"]) // 允许的 HTTP 方法
            .allowed_headers(vec![
                AUTHORIZATION,
                ACCEPT,
                ACCESS_CONTROL_ALLOW_ORIGIN,
                CONTENT_TYPE,
                ACCESS_CONTROL_REQUEST_HEADERS,
                ACCESS_CONTROL_ALLOW_HEADERS,
                ACCESS_CONTROL_MAX_AGE,
                ACCESS_CONTROL_EXPOSE_HEADERS,
                HeaderName::from_str("Identification").unwrap(), //自定义请求头
            ])
            .max_age(3600); // 设置 preflight 缓存的最大时间
                            //创建App
        App::new()
            //挂载数据JWT
            //.app_data(Data::new(storage.clone()))
            //.app_data(Data::new(jwt_ttl))
            //.app_data(Data::new(refresh_ttl))
            //JWT 中间件
            // .app_data(Data::new(redis.clone()))
            //跨域请求
            .wrap(cors)
            .wrap(VisiLog::default())
            .wrap(Logger::default()) //允许跨域请求
            //service层
            .service(index_controller::site)
            .service(blog_controller::blogs)
            .service(blog_controller::category)
            .service(blog_controller::blog)
            .service(blog_controller::tag)
            .service(archive_controller::archives)
            .service(moment_controller::moments)
            .service(about_controller::about)
            .service(friend_controller::get_friend)
            .service(comment_controller::get_comments)
            .service(blog_controller::check_blog_password)
            //admin
            .service(
                web::scope("/admin/")
                    .wrap(factory.clone()) //admin
                    .default_service(web::to(adminIndexController::default)),
            )
            .default_service(web::to(index_controller::default))
    })
    .bind(format!("{}:{}", conf.server.host, conf.server.port))?
    .run()
    .await
}
