use std::collections::HashMap;
use std::string::ToString;
use rbatis::{IPage, IPageRequest, Page};
use rbs::to_value;
use rbs::value::map::ValueMap;
use rbs::Value;
use crate::dao::blog_dao::{get_blog_list,get_by_category,get_blog_list_by_is_published as get_blog_public,get_by_tag,get_by_id as getById};
use crate::models::vo::{blog_info::BlogInfo,blog_detail::BlogDetail,blog_archive::BlogArchive};
use rand::Rng;
use crate::dao::blog_dao;
use crate::utils::markdown_parse::MarkdownParse;

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
        let  page_list:Page<BlogInfo>;
        if page_num.is_none(){
            page_list =match get_blog_public(1,PAGE_SIZE).await {
                Ok(ok)=>ok,
                Err(e)=> {
                    log::error!("BlogList查询失败:{:?}",e);
                    Page::new(0, 0)
                }
            };
        }else{
            page_list=match get_blog_public(page_num.expect("异常！"),PAGE_SIZE).await{
                Ok(ok)=>ok,
                Err(e)=> {
                    log::error!("BlogList查询失败:{:?}",e);
                    Page::new(0, 0)
                }
            }
        }
        map.insert("list".to_string(),to_value!(page_list.get_records()));
        map.insert("totalPage".to_string(),to_value!(page_list.pages()));
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
            log::error!("{}",e);
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
    let mut page_list:Page<BlogInfo>;
    page_list=match get_by_category(name ,page_num,PAGE_SIZE).await{
            Ok(ok)=>ok,
            Err(e)=> {
                log::error!("BlogList查询失败:{}",e);
                Page::new(0, 0)
            }
        };
    page_list.get_records_mut().iter_mut().for_each(|item|{
        item.description =MarkdownParse::to_html(&item.description);
        item.create_time=item.create_time.as_str()[0..19].to_string();
    });    
    map.insert("list".to_string(),to_value!(page_list.get_records()));
    map.insert("totalPage".to_string(),to_value!(page_list.pages()));
    map
}
//根据ID查找博文
pub(crate) async fn get_by_id(id: u16) -> Option<BlogDetail> {
    let mut blog=getById(id).await
    .unwrap_or_else(||{
        BlogDetail::new()
    });
    blog.content=MarkdownParse::to_html(&blog.content);
    Some(blog)
}

//根据tag名称查询博文
pub async fn get_by_tag_name(name :String,page_num:usize) ->HashMap<String, Value>{
    let mut map :HashMap<String, Value>=HashMap::new();
    let mut page_list:Page<BlogInfo>;
    page_list=match get_by_tag(name ,page_num,PAGE_SIZE).await{
            Ok(ok)=>ok,
            Err(e)=> {
                log::error!("BlogList查询失败:{}",e);
                Page::new(0, 0)
            }
        };
        page_list.get_records_mut().iter_mut().for_each(|item|{
            item.description =MarkdownParse::to_html(&item.description);
            item.create_time=item.create_time.as_str()[0..19].to_string();
            
        });    
    map.insert("list".to_string(),to_value!(page_list.get_records()));
    map.insert("totalPage".to_string(),to_value!(page_list.pages()));
    map
}

//获取归档文章
pub(crate) async fn get_archives()->ValueMap{
    //获取所有文章的日期
    let mut map :ValueMap=ValueMap::new();
    let blog_datetimes=blog_dao::get_all_datetime().await.unwrap_or_else(|e|{
        log::error!("{:?}",e);
        vec![]
    });
    let mut date_times = vec![];
    let _ =blog_datetimes.iter()
    .map(|itme|{
        date_times.push(itme.create_time.format("YYYY-MM"))
    }).collect::<Vec<_>>();
    //通过日期获取文章
    //todo!("！！！！未完成");
    for item in date_times.iter_mut(){
        let mut itme_map:Vec<BlogArchive> =vec![];
        
        let blogs =blog_dao::get_by_date(item.clone()).await.unwrap_or_else(|e|{
            log::error!("{:?}",e);
            vec![]
        });
        for blog in blogs{
            let mut  blog_archive=BlogArchive::new();
            blog_archive.id=blog.id.unwrap().to_string();
            blog_archive.password=blog.password.unwrap_or_default();
            blog_archive.privacy=false;
            blog_archive.day=blog.create_time.as_str()[8..10].to_string()+"日";
            blog_archive.title=blog.title;
            itme_map.push(blog_archive);
        }
        //更改
        item.insert(4, '年');
        item.insert(10, '月');
        map.insert(to_value!(item), to_value!(itme_map));
    }
    map
}

pub(crate) async fn get_archives_count()->Option<usize>{
    Some(blog_dao::get_archives_count().await.unwrap() as usize)
}

#[cfg(test)]
mod tests{
    use rbatis::rbdc::DateTime;

    #[test]
    pub(crate) fn test_datetime(){
        let time =DateTime::now().format("YYYY-MM-DD hh:mm:ss").parse::<DateTime>().unwrap();
        println!("{:?}",time)

    }
}
