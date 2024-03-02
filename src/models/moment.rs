use serde::{Deserialize, Serialize};
use rbatis::{crud, impl_select_page};
use rbatis::rbdc::datetime::DateTime;
#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct Moment{
    id:Option<u64>,
    content:String,
    create_time:DateTime,
    likes:u64,
    is_published:u8,
}
crud!(Moment {});
impl_select_page!(Moment{select_page() => "`where is_published = 1`"});