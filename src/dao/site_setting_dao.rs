use async_trait::async_trait;
use rbatis::crud::CRUD;
use crate::dao::site_setting_dao::SiteSettingDao;
use crate::models::site_setting::{SiteSet, SiteSetting};
use crate::rbatis::init_rbatis;

pub struct SiteSettingDaoImpl();

#[async_trait]
impl SiteSettingDao for SiteSettingDaoImpl {
    async fn get_list(self) -> Vec<SiteSetting> {
        let SiteSettings= init_rbatis().await.fetch_list().await;
        let v =match SiteSettings {
            Ok(v)=>v,
            Err(..)=>vec!(),
        };
        v
    }
}

impl SiteSettingDaoImpl{
    pub fn new()->SiteSettingDaoImpl{
        SiteSettingDaoImpl{}
    }
}
