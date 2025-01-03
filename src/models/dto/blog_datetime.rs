use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
//博文日期时间映射结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogDateTime {
    pub create_time: NaiveDateTime,
}
