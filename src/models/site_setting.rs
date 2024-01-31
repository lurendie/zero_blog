use rbatis::crud_table;
use serde::{Deserialize, Serialize};

#[crud_table(table_name: "site_setting" | table_columns:"id,name_en,name_zh,value,type")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SiteSetting{
    //#[crud_table(column: "id")]
    pub id:Option<u16>,
    //#[crud_table(column: "name_en")]
    pub name_en :String,
    //#[crud_table(column: "name_zh")]
    pub name_zh :String,
    //#[crud_table(column: "value")]
    pub value :String,
    pub r#type :u16 //1基础设置，2页脚徽标，3资料卡，4友链信息
}
