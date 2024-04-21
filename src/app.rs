use crate::config::Config;
use crate::controller::admin::index_controller as adminIndexController;
use crate::controller::{
    about_controller, archive_controller, blog_controller, comment_controller, friend_controller,
    index_controller, moment_controller,
};
use actix_cors::Cors;
use actix_web::http::header::{
    ACCEPT, ACCESS_CONTROL_ALLOW_ORIGIN, ACCESS_CONTROL_MAX_AGE, ACCESS_CONTROL_REQUEST_HEADERS,
    AUTHORIZATION, CONTENT_TYPE,
};
use actix_web::{web, App, HttpServer};

/**
 * 服务启动
 * return io
 */
pub async fn run(conf: &'static Config) -> std::io::Result<()> {
    HttpServer::new(|| {
        // 配置 CORS
        let cors = Cors::default()
            .allowed_origin(conf.server.front_adderss.as_str()) // 只允许 example.com 域进行跨域请求,暂时不开启
            //.allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"]) // 允许的 HTTP 方法
            .allowed_headers(vec![
                AUTHORIZATION,
                ACCEPT,
                ACCESS_CONTROL_ALLOW_ORIGIN,
                CONTENT_TYPE,
                ACCESS_CONTROL_REQUEST_HEADERS,
                ACCESS_CONTROL_MAX_AGE,
            ])
            .max_age(3600); // 设置 preflight 缓存的最大时间

        App::new()
            //跨域请求
            .wrap(cors) //允许跨域请求
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
            //admin
            .service(
                web::scope("/admin/") //admin
                    .default_service(web::to(adminIndexController::default)),
            )
            .default_service(web::to(index_controller::default))
    })
    .bind(format!("{}:{}", conf.server.host, conf.server.port))?
    .run()
    .await
}
