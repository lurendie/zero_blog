use std::collections::HashMap;
use std::string::ToString;
use rbatis::{Page};
use serde_json::{json, Value};
use crate::dao::blog_dao::{get_blog_list,get_by_name as getByname,get_blog_list_by_is_published as get_blog_public};
use crate::models::vo::blog_info::BlogInfo;
use rand::Rng;

//随机博客显示5条
const RANDOM_BLOG_LIMIT_NUM:u64= 5;
//最新推荐博客显示3条
const NEW_BLOG_PAGE_SIZE:u64= 3;
//每页显示5条博客简介
const PAGE_SIZE:u64= 5;
const _ORDER_BY:&str=  "is_top desc, create_time desc";

const   _PRIVATE_BLOG_DESCRIPTION :&str="此文章受密码保护！";

    pub(crate) async fn get_blog_list_by_is_published(page_num:Option<u64>) -> HashMap<String, Value> {
        let mut map :HashMap<String, Value>=HashMap::new();
        let page_list:Page<BlogInfo>;
        if page_num.is_none(){
            page_list =match get_blog_public(1,PAGE_SIZE).await {
                Ok(ok)=>ok,
                Err(e)=> {

                    log::error!("BlogList查询失败");
                    panic!("{}",e)
                }
            };
        }else{
            page_list=match get_blog_public(page_num.expect("异常！"),PAGE_SIZE).await{
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
//随机文章
pub async fn get_blog_list_random()->Result<Vec<BlogInfo>,rbatis::Error>{
    match get_blog_list().await{
        Ok(list)=> {
            //计数
            let mut count =0;
            let mut rng = rand::thread_rng();
            Ok(list.clone().into_iter().filter(|_|{
                //随机RANDOM_BLOG_LIMIT_NUM次,超过则不在进行添加的
                if count<RANDOM_BLOG_LIMIT_NUM {
                    count+=1;
                    list[rng.gen_range(1..list.len())].id.expect("异常")as usize >0
                }else{
                    false
                }
            }).collect())
        },
        Err(e)=>{
            log::error!("{}",e.to_string());
           Err(e)
        }
    }
}

//newBlog 
pub async fn get_blog_list_new()->Result<Vec<BlogInfo>,rbatis::Error>{
    match get_blog_list().await{
        Ok(list)=> {
            //计数
            let mut count =0;
            Ok(list.clone().into_iter().filter(|_|{
                //NEW_BLOG_PAGE_SIZE,超过则不在进行添加的
                if count<NEW_BLOG_PAGE_SIZE {
                    count+=1;
                    true
                }else{
                    false
                }
            }).collect())
        },
        Err(e)=>{
            log::error!("{}",e.to_string());
           Err(e)
        }
    }
}

//根据分类名称查询博文
pub async fn get_by_name(name :String,page_num:usize) ->HashMap<String, Value>{
    let mut map :HashMap<String, Value>=HashMap::new();
    let page_list:Page<BlogInfo>;
    page_list=match getByname(name ,page_num,PAGE_SIZE).await{
            Ok(ok)=>ok,
            Err(e)=> {
                log::error!("BlogList查询失败");
                panic!("{}",e)
            }
        };
    map.insert("list".to_string(),json!(page_list.records));
    map.insert("totalPage".to_string(),json!(page_list.pages));
    map
}