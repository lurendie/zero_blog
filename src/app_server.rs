/*
 * @Author: lurendie 549700459@qq.com
 * @Date: 2024-03-26 00:08:12
 * @LastEditors: lurendie
 */
use crate::app_state::{self, AppState};
use crate::config::CONFIG;
use crate::controller::{
    about_controller, admin, archive_controller, blog_controller, comment_controller,
    friend_controller, index_controller, moment_controller, user_controller,
};
use crate::middleware::{AppClaims, VisiLog, JWT};
use actix_jwt_session::{Duration, JwtTtl, RefreshTtl};
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};

pub struct AppServer;

/**
 * Application Server Implementation
 *
 */
impl AppServer {
    /**
     * run 服务启动
     */
    pub async fn run() -> std::io::Result<()> {
        //创建JWT
        let jwt_ttl = JwtTtl(Duration::days(CONFIG.server.token_expires));
        let refresh_ttl = RefreshTtl(Duration::days(CONFIG.server.token_expires));
        //Appstate
        let app_state = AppState::new(
            app_state::get_connection().await,
            // redis::get_redis_pool().await,
            // CONFIG.clone(),
        );

        HttpServer::new(move || {
            //创建App
            App::new()
                .app_data(Data::new(jwt_ttl))
                .app_data(Data::new(refresh_ttl))
                .app_data(Data::new(app_state.clone()))
                .wrap(VisiLog::default())
                .wrap(Logger::default()) //允许跨域请求
                .configure(Self::view_router)
                //admin
                .configure(Self::admin_router)
                .default_service(web::to(index_controller::default))
        })
        .bind(format!("{}:{}", CONFIG.server.host, CONFIG.server.port))?
        .run()
        .await
    }
    /**
     * 前台路由
     */
    fn view_router(cfg: &mut web::ServiceConfig) {
        //service层
        cfg.service(index_controller::site)
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
            .service(user_controller::login)
            .service(blog_controller::search_blog)
            .service(moment_controller::moment_like);
    }

    /**
     * 后台路由
     */
    fn admin_router(cfg: &mut web::ServiceConfig) {
        let (storage, factory) = JWT::create::<AppClaims>();
        cfg.service(
            web::scope("/admin")
                .app_data(Data::new(storage.clone()))
                .wrap(factory.clone())
                .service(user_controller::login)
                .service(admin::dashboard_controller::dashboard) //.default_service(web::to(adminIndexController::default)),
                .service(admin::blog_controller::blogs)
                .service(admin::blog_controller::visibility)
                .service(admin::blog_controller::top)
                .service(admin::blog_controller::recommend)
                .service(admin::blog_controller::category_and_tag)
                .service(admin::blog_controller::blog)
                .service(admin::blog_controller::update_blog)
                .service(admin::blog_controller::create_blog)
                .service(admin::blog_controller::delete_blog)
                .service(admin::moment_controller::moments)
                .service(admin::moment_controller::moment_published)
                .service(admin::moment_controller::delete_moment)
                .service(admin::moment_controller::get_moment_by_id)
                .service(admin::moment_controller::update_moment)
                .service(admin::moment_controller::create_moment)
                .service(admin::category_controller::categories)
                .service(admin::category_controller::update_category)
                .service(admin::category_controller::delete_category)
                .service(admin::tag_controller::get_all_tags),
        );
    }
}
