use actix_jwt_session::deadpool_redis;
use actix_jwt_session::Pair;

use actix_jwt_session::SessionMiddlewareFactory;
use actix_jwt_session::SessionStorage;
use actix_jwt_session::JWT_COOKIE_NAME;
use actix_jwt_session::REFRESH_COOKIE_NAME;
use actix_jwt_session::REFRESH_HEADER_NAME;
use actix_web::HttpResponse;
/*
 * @Author: lurendie 549700459@qq.com
 * @Date: 2024-04-24 23:25:31
 * @LastEditors: lurendie
 * @LastEditTime: 2024-05-06 23:22:15
 * @FilePath: \zero_blog\src\middleware\jwt.rs
 */
use serde::Deserialize;

#[derive(Deserialize)]
struct MyJsonBody {
    jwt: Option<String>,
    refresh: Option<String>,
}

use serde::Serialize;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Audience {
    Web,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
#[serde(rename_all = "snake_case")]
pub struct AppClaims {
    #[serde(rename = "exp")]
    pub expiration_time: u64,
    #[serde(rename = "iat")]
    pub issues_at: usize,
    // Account login
    #[serde(rename = "username")]
    pub subject: String,
    // #[serde(rename = "aud")]
    // pub audience: Audience,
    #[serde(rename = "jti")]
    pub jwt_id: actix_jwt_session::Uuid,
    #[serde(rename = "aci")]
    pub account_id: i32,
    #[serde(rename = "nbf")]
    pub not_before: u64,
}

impl actix_jwt_session::Claims for AppClaims {
    fn jti(&self) -> actix_jwt_session::Uuid {
        self.jwt_id
    }

    fn subject(&self) -> &str {
        &self.subject
    }
}

impl AppClaims {
    pub fn account_id(&self) -> i32 {
        self.account_id
    }
}

pub static JWT_HEADER_NAME: &str = "Authorization";

pub async fn create_response<C: actix_jwt_session::Claims>(pair: Pair<C>) -> HttpResponse {
    let jwt_text = pair.jwt.encode().unwrap();
    let refresh_text = pair.refresh.encode().unwrap();
    HttpResponse::Ok()
        .append_header((JWT_HEADER_NAME, jwt_text.as_str()))
        .append_header((REFRESH_HEADER_NAME, refresh_text.as_str()))
        .cookie(actix_web::cookie::Cookie::build(JWT_COOKIE_NAME, jwt_text.as_str()).finish())
        .cookie(
            actix_web::cookie::Cookie::build(REFRESH_COOKIE_NAME, refresh_text.as_str()).finish(),
        )
        .finish()
}

pub fn create<AppClaims: actix_jwt_session::Claims>(
) -> (SessionStorage, SessionMiddlewareFactory<AppClaims>) {
    let redis = deadpool_redis::Config::from_url("redis://localhost:6379")
        .create_pool(Some(deadpool_redis::Runtime::Tokio1))
        .unwrap();
    // create new [SessionStorage] and [SessionMiddlewareFactory]
    let (storage, factory) = SessionMiddlewareFactory::<AppClaims>::build_ed_dsa()
        // pass redis connection
        .with_redis_pool(redis.clone())
        // Check if header "Authorization" exists and contains Bearer with encoded JWT
        .with_jwt_header("Authorization")
        // Check if cookie "jwt" exists and contains encoded JWT
        .with_jwt_cookie("acx-a")
        .with_refresh_header("ACX-Refresh")
        // Check if cookie "jwt" exists and contains encoded JWT
        .with_refresh_cookie("acx-r")
        .finish();
    (storage, factory)
}
