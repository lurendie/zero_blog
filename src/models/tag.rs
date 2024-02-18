use serde::{Deserialize, Serialize};
use rbatis::crud;
use rbs;
#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct Tag{
    pub id :Option<u16>,
    pub name:String,
    pub color:String
}
crud!(Tag {});