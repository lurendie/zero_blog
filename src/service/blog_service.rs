use std::collections::HashMap;
use std::string::ToString;
use async_trait::async_trait;
use rbatis::{Page};
use serde_json::{json, Value};
use crate::dao::blog_dao::BlogDao;
use crate::service::blog_service::BlogService;
use crate::dao::dao_impl::blog_dao_impl;
use crate::models::vo::blog_info::BlogInfo;

//随机博客显示5条
const RANDOM_BLOG_LIMIT_NUM:u64= 5;

//最新推荐博客显示3条
const NEW_BLOG_PAGE_SIZE:u64= 3;

//每页显示5条博客简介
const PAGE_SIZE:u64= 5;
const ORDER_BY:&str=  "is_top desc, create_time desc";

const   PRIVATE_BLOG_DESCRIPTION :&str="此文章受密码保护！";

    async fn get_blog_list_by_is_published(page_Num:Option<u64>) -> HashMap<String, Value> {
        let mut map :HashMap<String, Value>=HashMap::new();
        let page_list:Page<BlogInfo>;
        if page_Num.is_none(){
            page_list =match blog_dao_impl::BlogDaoImpl::get_blog_list_by_is_published(1,PAGE_SIZE).await {
                Ok(ok)=>ok,
                Err(e)=> {

                    log::error!("BlogList查询失败");
                    panic!("{}",e)
                }
            };
        }else{
            page_list=match blog_dao_impl::BlogDaoImpl::get_blog_list_by_is_published(page_Num.expect("异常！"),PAGE_SIZE).await{
                Ok(ok)=>ok,
                Err(e)=> {
                    log::error!("BlogList查询失败");
                    panic!("{}",e)
                }
            }
        }
        map.insert("list".to_string(),json!(page_list.records));
        map.insert("totalPage".to_string(),json!(page_list.pages));
        map
    }