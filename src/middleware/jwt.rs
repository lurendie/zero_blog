use actix_jwt_session::deadpool_redis::Pool;
use actix_jwt_session::SessionMiddlewareFactory;
use actix_jwt_session::SessionStorage;
/*
 * @Author: lurendie 549700459@qq.com
 * @Date: 2024-04-24 23:25:31
 * @LastEditors: lurendie
 * @LastEditTime: 2024-04-26 22:45:01
 * @FilePath: \zero_blog\src\utils\jwt.rs
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
pub struct Claims {
    #[serde(rename = "exp")]
    pub expiration_time: u64,
    #[serde(rename = "iat")]
    pub issues_at: usize,
    /// Account login
    #[serde(rename = "sub")]
    pub subject: String,
    #[serde(rename = "aud")]
    pub audience: Audience,
    #[serde(rename = "jti")]
    pub jwt_id: actix_jwt_session::Uuid,
    #[serde(rename = "aci")]
    pub account_id: i32,
    #[serde(rename = "nbf")]
    pub not_before: u64,
}

impl actix_jwt_session::Claims for Claims {
    fn jti(&self) -> actix_jwt_session::Uuid {
        self.jwt_id
    }

    fn subject(&self) -> &str {
        &self.subject
    }
}

impl Claims {
    pub fn account_id(&self) -> i32 {
        self.account_id
    }
}

pub fn create<AppClaims: actix_jwt_session::Claims>(
    redis: &Pool,
) -> (SessionStorage, SessionMiddlewareFactory<AppClaims>) {
    // create new [SessionStorage] and [SessionMiddlewareFactory]
    let (storage, factory) = SessionMiddlewareFactory::<AppClaims>::build_ed_dsa()
        // pass redis connection
        .with_redis_pool(redis.clone())
        // Check if header "Authorization" exists and contains Bearer with encoded JWT
        .with_jwt_header("Authorization")
        // Check if cookie "jwt" exists and contains encoded JWT
        //.with_jwt_cookie("acx-a")
        //.with_refresh_header("ACX-Refresh")
        // Check if cookie "jwt" exists and contains encoded JWT
        //.with_refresh_cookie("acx-r")
        .finish();
    (storage, factory)
}
