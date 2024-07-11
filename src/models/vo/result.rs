use std::fmt::Debug;

/*
 * @Author: lurendie 549700459@qq.com
 * @Date: 2024-02-24 22:58:03
 * @LastEditors: lurendie
 * @LastEditTime: 2024-05-14 19:27:50
 */
use actix_web::{
    http::{header, StatusCode},
    HttpResponse,
};
use rbs::Value;
//封装响应结果
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Result<T> {
    pub code: u16,
    pub msg: String,
    pub data: Option<T>,
}

impl<T> Result<T> {
    pub fn new(code: u16, msg: String, data: Option<T>) -> Result<T> {
        Result { code, msg, data }
    }

    //无异常返回
    pub fn ok(msg: String, data: Option<T>) -> Result<T> {
        Result {
            code: 200,
            msg,
            data,
        }
    }

    //无异常返回数据
    pub fn ok_no_data(msg: String) -> Result<T> {
        Result {
            code: 200,
            msg,
            data: None,
        }
    }

    pub fn error(msg: String) -> Result<T> {
        Result {
            code: 500,
            msg,
            data: None,
        }
    }
}
//针对于ValueMap具体实现
impl Result<Value> {
    pub fn new_value(code: u16, msg: String, data: Option<Value>) -> Result<Value> {
        Result { code, msg, data }
    }
    //无异常返回
    pub fn ok_json(&self) -> HttpResponse {
        HttpResponse::Ok()
            .insert_header(header::ContentType(mime::APPLICATION_JSON))
            .json(&self)
    }

    pub fn error_json(&self) -> HttpResponse {
        HttpResponse::Ok()
            .insert_header(header::ContentType(mime::APPLICATION_JSON))
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .json(&self)
    }
}

impl<T: Debug> std::fmt::Display for Result<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {},{:?})", self.code, self.msg, self.data)
    }
}
