use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataBaseError {
    #[error("redis中发生错误，异常原因是：{0}")]
    RedisError(#[from] redis::RedisError),
    //#[error("mysql 错误原因是：{0}")]
    //MySQLError(String),
    #[error("unknown data store error")]
    Unknown,
}
