use rbatis::crud_table;
use rbson;
//关于
#[crud_table]
#[derive(Debug, Clone)]
pub struct About{
    id:Option<u16>,
    name_en :String,
    name_zh :String,
    value :String,
}