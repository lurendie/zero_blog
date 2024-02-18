use serde::{Deserialize, Serialize};
use rbatis::{crud};
use rbs;
#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct Category{
    pub id:u16,
    pub name:String
}

crud!(Category {});