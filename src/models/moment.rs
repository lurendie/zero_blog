use crate::service::CommonService;
use rbatis::{crud, impl_select_page};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize,Default)]
pub struct Moment {
    pub(crate) id: Option<u64>,
    pub(crate) content: String,
    #[serde(rename(serialize = "createTime"))]
    pub(crate) create_time: String,
    pub(crate) likes: u64,
    #[serde(
        rename(serialize = "published"),
        deserialize_with = "CommonService::bool_from_int"
    )]
    pub(crate) is_published: bool,
}
crud!(Moment {}, "moment");
impl_select_page!(Moment{select_page() => "`order by id desc`"});
impl_select_page!(Moment{select_published_page() => "`where is_published = 1 order by id desc`"});
