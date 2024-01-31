use rbatis::crud_table;
use serde::{Deserialize, Serialize};

#[crud_table(table_name: "tag")]
#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct Tag{
    pub id :Option<u16>,
    pub name:String,
    pub color:String
}