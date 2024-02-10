use std::collections::HashMap;
use rbs::Value;
use rbs::to_value;
use serde_json::json;
use crate::constant::site_setting_constants;
use crate::dao;
use crate::models::vo::{introduction,badge::Badge,copyright::Copyright,favorite::Favorite};
    pub async fn get_site_info() ->HashMap<String, Value> {
        let site_setting_list = dao::site_setting_dao::get_list().await; // 假设这是一个 Vec 或其他可迭代集合
        let mut map:HashMap<String, Value> = HashMap::new();
        let mut introduction = introduction::Introduction::new();
        let mut site_info:HashMap<String, Value> = HashMap::new();
        let mut badges = vec![];
       // let mut rollTexts = vec![];
        let mut favorites:Vec<Favorite> = vec![];
        for v in site_setting_list {
            match v.r#type {
                //类型1
                1 => {
                    if  site_setting_constants::COPYRIGHT == v.name_en{
                       let copyright:Copyright=serde_json::from_str(v.value.as_str()).unwrap();
                        site_info.insert(v.name_en, to_value!(copyright));
                    }else{
                        site_info.insert(v.name_en, Value::String(v.value));
                    }
                },
                //类型2
                2 =>
                    match v.name_en.as_str() {
                        site_setting_constants::AVATAR => introduction.avatar = v.value,
                        site_setting_constants::NAME => introduction.name = v.value,
                        site_setting_constants::GITHUB => introduction.github = v.value,
                        site_setting_constants::TELEGRAM => introduction.telegram = v.value,
                        site_setting_constants::QQ => introduction.qq = v.value,
                        site_setting_constants::BILIBILI => introduction.bilibili = v.value,
                        site_setting_constants::NETEASE => introduction.netease = v.value,
                        site_setting_constants::EMAIL => introduction.email = v.value,
                        site_setting_constants::FAVORITE => {
                            let favorite=serde_json::from_str(v.value.as_str()).unwrap();
                            favorites.push(favorite);
                        },
                        site_setting_constants::ROLL_TEXT => {
                            let arr  =v.value.split(",").map(String::from).collect();
                            introduction.rollText=arr;
                        }
                        ,
                        _ => ()
                    }
                ,
                //类型1
                3 =>{
                    let badge:Badge=serde_json::from_str(v.value.as_str()).expect("异常");
                    badges.push(badge);
                },
                _ => (),
            }
        }
        introduction.favorites=favorites;
        map.insert("introduction".to_string(),to_value!(introduction));
        map.insert( "siteInfo".to_string(),to_value!(site_info));
        map.insert("badges".to_string(),to_value!(badges));
        map
    }