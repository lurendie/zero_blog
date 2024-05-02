/*
 * @Author: lurendie 549700459@qq.com
 * @Date: 2024-04-30 00:04:06
 * @LastEditors: lurendie
 * @LastEditTime: 2024-05-03 00:34:26
 * @FilePath: \zero_blog\src\middleware\visit_log.rs
 */
use std::{
    future::{ready, Future, Ready},
    pin::Pin,
    str::FromStr,
};

use actix_jwt_session::Uuid;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::header::{HeaderName, HeaderValue},
    Error,
};
/**
 * 校验访客标识码
 */
#[derive(Default, Debug)]
pub struct VisiLog;

impl<S, B> Transform<S, ServiceRequest> for VisiLog
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = VisitLogMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(VisitLogMiddleware { service }))
    }
}

pub struct VisitLogMiddleware<S> {
    /// The next service to call
    service: S,
}

// This future doesn't have the requirement of being `Send`.
// See: futures_util::future::LocalBoxFuture
type LocalBoxFuture<T> = Pin<Box<dyn Future<Output = T> + 'static>>;

// `S`: type of the wrapped service
// `B`: type of the body - try to be generic over the body where possible
impl<S, B> Service<ServiceRequest> for VisitLogMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<Result<Self::Response, Self::Error>>;

    // This service is ready when its next service is ready
    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let fut = self.service.call(req);

        Box::pin(async move {
            let mut res = fut.await?;
            //1.检测访客标识码是否存在
            let req_headers = res.request().headers();
            let identification = req_headers.get("Identification");
            if let Some(uuid) = identification {
                log::info!("访客UUID:{:?}", uuid)
            } else {
                let resp = res.response_mut();
                let resp_headers = resp.headers_mut();
                //添加访客标识码UUID至响应头
                resp_headers.insert(
                    HeaderName::from_str("Identification").unwrap(),
                    HeaderValue::from_str(Uuid::new_v4().to_string().as_str()).unwrap(),
                );
                resp_headers.insert(
                    HeaderName::from_str("access-control-expose-headers").unwrap(),
                    HeaderValue::from_str("Identification").unwrap(),
                );
            }
            Ok(res)
        })
    }
}
