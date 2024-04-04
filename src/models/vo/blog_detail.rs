use crate::models::category::Category;
use rbatis::rbdc::datetime::DateTime;
use rbatis::{crud, impl_select};
use serde::de::Unexpected;
use serde::{Deserialize, Deserializer, Serialize};
//博客详情信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BlogDetail {
    pub(crate) id: Option<u16>,
    pub(crate) title: String,
    pub(crate) content: String,
    #[serde(deserialize_with = "bool_from_int")]
    pub(crate) is_appreciation: bool,
    #[serde(
        rename(serialize = "commentEnabled"),
        deserialize_with = "bool_from_int"
    )]
    pub(crate) is_comment_enabled: bool,
    #[serde(rename(serialize = "createTime"))]
    pub(crate) create_time: DateTime,
    #[serde(rename(serialize = "updateTime"))]
    pub(crate) update_time: DateTime,
    pub(crate) views: u16,
    pub(crate) words: u16,
    pub(crate) read_time: u16,
    #[serde(deserialize_with = "bool_from_int")]
    pub(crate) is_top: bool,
    pub(crate) category: Option<Category>,
    pub(crate) password: Option<String>,
}

impl BlogDetail {
    pub(crate) fn new() -> Self {
        BlogDetail::default()
    }
}

fn bool_from_int<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match i32::deserialize(deserializer)? {
        0 => Ok(false),
        1 => Ok(true),
        other => Err(serde::de::Error::invalid_value(
            Unexpected::Unsigned(other.try_into().unwrap()),
            &"0 or 1",
        )),
    }
}

crud!(BlogDetail {}, "blog");
impl_select!(BlogDetail{select_by_id(id:String) -> Option => "`where id = #{id} limit 1`"},"blog");
