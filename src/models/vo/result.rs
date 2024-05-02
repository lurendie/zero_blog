use actix_web::{http::header, HttpResponse};
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
            msg: msg,
            data: data,
        }
    }

    //无异常返回数据
    pub fn ok_no_data(msg: String) -> Result<T> {
        Result {
            code: 200,
            msg: msg,
            data: None,
        }
    }

    pub fn error(msg: String) -> Result<T> {
        Result {
            code: 500,
            msg: msg,
            data: None,
        }
    }
}
//针对于ValueMap具体实现
impl Result<Value> {
    //无异常返回
    pub fn ok_json(&self) -> HttpResponse {
        HttpResponse::Ok()
            .insert_header(header::ContentType(mime::APPLICATION_JSON))
            .json(&self)
    }

    pub fn error_json(&self) -> HttpResponse {
        HttpResponse::Ok()
            .insert_header(header::ContentType(mime::APPLICATION_JSON))
            .json(&self)
    }
}
