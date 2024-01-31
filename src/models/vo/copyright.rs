use serde::{Deserialize, Serialize};

#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct Copyright{
    pub title:String,
    pub siteName:String
}