use rbatis::crud::CRUD;
use crate::models::site_setting::{SiteSetting};
use crate::rbatis::RBATIS;

    pub async fn get_list() -> Vec<SiteSetting> {
        let site_settings = RBATIS.fetch_list().await;
        let v =match site_settings {
            Ok(v)=>v,
            Err(..)=>vec!(),
        };
        v
    }
