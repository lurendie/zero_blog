use crate::models::category::Category;
use crate::dao::category_dao::{get_list as getList};

pub async fn get_list() ->Vec<Category>{
    getList().await
}

