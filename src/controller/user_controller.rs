/*
 * @Author: lurendie
 * @Date: 2024-05-03 23:58:25
 * @LastEditors: lurendie
 * @LastEditTime: 2024-05-16 13:14:29
 *
 */

use crate::models::Result;
use crate::{middleware::AppClaims, service::user_service};
use actix_jwt_session::JWT_COOKIE_NAME;
use actix_jwt_session::{
    JwtTtl, OffsetDateTime, RefreshTtl, SessionStorage, Uuid, JWT_HEADER_NAME, REFRESH_HEADER_NAME,
};
use actix_web::{
    routes,
    web::{Data, Json},
    HttpResponse, Responder,
};
use rbs::value::map::ValueMap;
use rbs::{to_value, Value};
use serde::Deserialize;

#[derive(Deserialize)]
struct SignInPayload {
    username: String,
    password: String,
}

#[routes]
#[post("/login")]
pub async fn login(
    user_form: Json<SignInPayload>,
    store: Data<SessionStorage>,
    jwt_ttl: Data<JwtTtl>,
    refresh_ttl: Data<RefreshTtl>,
) -> impl Responder {
    //验证账号 密码是否正确
    let mut user = user_service::get_by_username(&user_form.username).await;
    if let Some(user) = user.as_mut() {
        if user_form.password != user.get_password() || user.get_role() != "ROLE_admin" {
            return HttpResponse::Unauthorized().finish();
        }
        let uuid = Uuid::new_v4();
        //创建认证数据
        let claims = AppClaims {
            issues_at: OffsetDateTime::now_utc().unix_timestamp() as usize,
            subject: user.get_username(),
            expiration_time: jwt_ttl.0.as_seconds_f64() as u64,
            //audience: Audience::Web,
            jwt_id: Uuid::parse_str(uuid.to_string().as_str()).unwrap(),
            account_id: user.get_id(),
            not_before: 0,
        };
        let pair = store
            .clone()
            .store(claims, *jwt_ttl.into_inner(), *refresh_ttl.into_inner())
            .await
            .unwrap();

        let mut map: ValueMap = ValueMap::new();
        user.set_password("".to_string());
        map.insert(to_value!("user"), to_value!(user));
        map.insert(to_value!("token"), to_value!(pair.jwt.encode().unwrap()));
        let result = Result::<Value>::ok("请求成功".to_string(), Some(to_value!(map)));
        return HttpResponse::Ok()
            .append_header((JWT_HEADER_NAME, pair.jwt.encode().unwrap()))
            .append_header((REFRESH_HEADER_NAME, pair.refresh.encode().unwrap()))
            .cookie(
                actix_web::cookie::Cookie::build(JWT_COOKIE_NAME, pair.jwt.encode().unwrap())
                    .finish(),
            )
            .json(result);
    }
    HttpResponse::Unauthorized().finish()
}
