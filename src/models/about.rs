use rbatis::crud;
use serde::{Deserialize, Serialize};
use rbs;
//关于
#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct About{
    id:Option<u16>,
    name_en :String,
    name_zh :String,
    value :String,
}
crud!(About {});