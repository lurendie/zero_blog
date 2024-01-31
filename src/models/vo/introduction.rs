use serde::{Deserialize, Serialize};
use crate::models::vo::favorite::Favorite;
//侧边资料卡
#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct Introduction{
    pub avatar:String,
    pub name:String,
    pub github:String,
    pub telegram:String,
    pub qq:String,
    pub bilibili :String,
    pub netease :String,
    pub email :String,
    pub rollText:Vec<String>,
    pub favorites:Vec<Favorite>
}

impl Introduction{
    pub fn new() ->Introduction{
        Introduction{
            avatar: "".to_string(),
            name: "".to_string(),
            github: "".to_string(),
            telegram: "".to_string(),
            qq: "".to_string(),
            bilibili: "".to_string(),
            netease: "".to_string(),
            email: "".to_string(),
            rollText: vec![],
            favorites: vec![],
        }
    }
}