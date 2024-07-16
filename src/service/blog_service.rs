use crate::constant::blog_info_constants;
use crate::constant::redis_key_constants;
use crate::dao::BlogDao;
use crate::dao::{CategoryDao, TagDao};
use crate::models::dto::blog_dto::BlogDto;
use crate::models::vo::blog_visibility::BlogVisibility;
use crate::models::vo::blog_vo::BlogVO;
use crate::models::vo::page_request::SearchRequest;
use crate::models::vo::{blog_archive::BlogArchive, blog_detail::BlogDetail, blog_info::BlogInfo};
use crate::rbatis::RBATIS;
use crate::service::RedisService;
use rand::Rng;
use rbatis::rbdc::DateTime;
use rbatis::{IPage, IPageRequest, Page};
use rbs::to_value;
use rbs::value::map::ValueMap;
use rbs::Value;
use std::collections::HashMap;

use super::BlogTagService;
use super::TagService;

pub struct BlogService;

impl BlogService {
    pub(crate) async fn get_blog_list_by_is_published(
        page_num: Option<u64>,
    ) -> HashMap<String, Value> {
        //println!("{:?}", page_num);
        let num;
        if let Some(page_num) = page_num {
            num = page_num;
        } else {
            num = 1;
        };
        //1.查询redis缓存
        let redis_cache = RedisService::get_hash_key(
            redis_key_constants::HOME_BLOG_INFO_LIST.to_string(),
            num.to_string(),
        )
        .await;
        //2.缓存不未Null则返回返回
        if let Some(redis_cache) = redis_cache {
            log::info!(
                "key:{} page:{} 数据存在",
                redis_key_constants::HOME_BLOG_INFO_LIST,
                num
            );
            return redis_cache;
        }
        //3.查询数据库
        let mut map: HashMap<String, Value> = HashMap::new();
        let page_list: Page<BlogInfo>;
        page_list = match BlogDao::get_blog_pages(num, blog_info_constants::PAGE_SIZE).await {
            Ok(mut ok) => {
                BlogService::bloginfo_handle(ok.get_records_mut()).await;
                ok
            }
            Err(e) => {
                log::error!("BlogList查询失败:{:?}", e);
                Page::new(0, 0)
            }
        };
        map.insert("list".to_string(), to_value!(page_list.get_records()));
        map.insert("totalPage".to_string(), to_value!(page_list.pages()));
        //4.如果数据库查询不是Null 存放到Redis中
        log::info!(
            "key:{} page:{} 数据不存在",
            redis_key_constants::HOME_BLOG_INFO_LIST,
            num
        );
        if !page_list.get_records().is_empty() {
            let _ = RedisService::set_hash_key(
                redis_key_constants::HOME_BLOG_INFO_LIST.to_string(),
                num.to_string(),
                &map,
            )
            .await;
        }
        map
    }
    /**
     * 获取随机文章
     */
    pub async fn get_blog_list_random() -> Vec<Value> {
        //1.查询Redis 缓存数据
        let redis_cache =
            RedisService::get_value_vec(redis_key_constants::RANDOM_BLOG_LIST.to_string()).await;
        if let Some(redis_cache) = redis_cache {
            let arr = match redis_cache {
                Value::Array(arr) => {
                    log::error!(
                        "key:{} 数据存在",
                        redis_key_constants::RANDOM_BLOG_LIST.to_string()
                    );
                    arr
                }
                _ => vec![],
            };
            return arr;
        }
        //2.查询数据库
        match BlogDao::get_blog_list().await {
            Ok(mut list) => {
                BlogService::bloginfo_handle(&mut list).await;
                let mut result = vec![];
                //计数
                let mut count = 0;
                let mut rng = rand::thread_rng();
                let _ = list
                    .clone()
                    .into_iter()
                    .filter(|_| {
                        //随机RANDOM_BLOG_LIMIT_NUM次,超过则不在进行添加的
                        if count < blog_info_constants::RANDOM_BLOG_LIMIT_NUM {
                            count += 1;
                            list[rng.gen_range(1..list.len())].id.expect("异常") as usize > 0
                        } else {
                            false
                        }
                    })
                    .collect::<Vec<_>>()
                    .iter()
                    .for_each(|item| {
                        result.push(to_value!(item));
                    });
                log::info!(
                    "key:{} 数据不存在",
                    redis_key_constants::RANDOM_BLOG_LIST.to_string()
                );
                //保存到Redis
                RedisService::set_value_vec(
                    redis_key_constants::RANDOM_BLOG_LIST.to_string(),
                    &to_value!(&result),
                )
                .await;
                return result;
            }
            Err(e) => {
                log::error!("{}", e);
                vec![]
            }
        }
    }

    /**
     * 获取最新文章
     */
    pub async fn get_blog_list_new() -> Vec<Value> {
        //1.查询Redis 缓存数据
        let redis_cache =
            RedisService::get_value_vec(redis_key_constants::NEW_BLOG_LIST.to_string()).await;
        if let Some(redis_cache) = redis_cache {
            let arr = match redis_cache {
                Value::Array(arr) => {
                    log::error!(
                        "key:{} 数据存在",
                        redis_key_constants::NEW_BLOG_LIST.to_string()
                    );
                    arr
                }
                _ => vec![],
            };
            return arr;
        }
        //2.查询数据库
        match BlogDao::get_blog_list().await {
            Ok(mut list) => {
                BlogService::bloginfo_handle(&mut list).await;
                let mut result = vec![];
                //计数
                let mut count = 0;
                list.clone()
                    .into_iter()
                    .filter(|_| {
                        //NEW_BLOG_PAGE_SIZE,超过则不在进行添加的
                        if count < blog_info_constants::NEW_BLOG_PAGE_SIZE {
                            count += 1;
                            true
                        } else {
                            false
                        }
                    })
                    .collect::<Vec<_>>()
                    .iter()
                    .for_each(|item| {
                        result.push(to_value!(item));
                    });
                log::info!(
                    "key:{} 数据不存在",
                    redis_key_constants::NEW_BLOG_LIST.to_string()
                );
                //保存到Redis
                RedisService::set_value_vec(
                    redis_key_constants::NEW_BLOG_LIST.to_string(),
                    &to_value!(&result),
                )
                .await;
                result
            }
            Err(e) => {
                log::error!("{}", e.to_string());
                vec![]
            }
        }
    }

    //根据分类名称查询博文
    pub async fn get_by_name(name: String, page_num: usize) -> HashMap<String, Value> {
        let mut map: HashMap<String, Value> = HashMap::new();
        let mut page_list: Page<BlogInfo>;
        page_list =
            match BlogDao::get_by_category(name, page_num, blog_info_constants::PAGE_SIZE).await {
                Ok(mut ok) => {
                    BlogService::bloginfo_handle(ok.get_records_mut()).await;
                    ok
                }
                Err(e) => {
                    log::error!("BlogList查询失败:{}", e);
                    Page::new(0, 0)
                }
            };
        page_list.get_records_mut().iter_mut().for_each(|item| {
            item.description = markdown::to_html(&item.description);
            item.create_time = item.create_time.as_str()[0..19].to_string();
        });
        map.insert("list".to_string(), to_value!(page_list.get_records()));
        map.insert("totalPage".to_string(), to_value!(page_list.pages()));
        map
    }
    //根据ID查找博文
    pub(crate) async fn get_by_id(id: u16) -> Option<BlogDetail> {
        let mut blog = BlogDao::get_by_id(id)
            .await
            .unwrap_or_else(|| BlogDetail::new());
        blog.content = markdown::to_html(&blog.content);
        Some(blog)
    }

    //根据tag名称查询博文
    pub async fn get_by_tag_name(name: String, page_num: usize) -> HashMap<String, Value> {
        let mut map: HashMap<String, Value> = HashMap::new();
        let mut page_list: Page<BlogInfo>;
        page_list = match BlogDao::get_by_tag(name, page_num, blog_info_constants::PAGE_SIZE).await
        {
            Ok(mut ok) => {
                BlogService::bloginfo_handle(ok.get_records_mut()).await;
                ok
            }
            Err(e) => {
                log::error!("BlogList查询失败:{}", e);
                Page::new(0, 0)
            }
        };
        page_list.get_records_mut().iter_mut().for_each(|item| {
            item.description = markdown::to_html(&item.description);
            item.create_time = item.create_time.as_str()[0..19].to_string();
        });
        map.insert("list".to_string(), to_value!(page_list.get_records()));
        map.insert("totalPage".to_string(), to_value!(page_list.pages()));
        map
    }

    //获取归档文章
    pub(crate) async fn get_archives() -> ValueMap {
        //获取所有文章的日期
        let mut map: ValueMap = ValueMap::new();
        let blog_datetimes = BlogDao::get_all_datetime().await.unwrap_or_else(|e| {
            log::error!("{:?}", e);
            vec![]
        });
        let mut date_times = vec![];
        let _ = blog_datetimes
            .iter()
            .map(|itme| date_times.push(itme.create_time.format("YYYY-MM")))
            .collect::<Vec<_>>();
        //通过日期获取文章
        for item in date_times.iter_mut() {
            let mut itme_map: Vec<BlogArchive> = vec![];

            let blogs = BlogDao::get_by_date(item.clone())
                .await
                .unwrap_or_else(|e| {
                    log::error!("{:?}", e);
                    vec![]
                });
            for blog in blogs {
                let mut blog_archive = BlogArchive::new();
                blog_archive.id = blog.id.unwrap().to_string();
                blog_archive.password = blog.password.unwrap_or_default();
                blog_archive.privacy = false;
                blog_archive.day = blog.create_time.as_str()[8..10].to_string() + "日";
                blog_archive.title = blog.title;
                itme_map.push(blog_archive);
            }
            //更改
            item.insert(4, '年');
            item.insert(10, '月');
            map.insert(to_value!(item), to_value!(itme_map));
        }
        map
    }

    pub(crate) async fn get_archives_count() -> Option<usize> {
        Some(BlogDao::get_archives_count().await.unwrap() as usize)
    }

    /**
     * 处理BlogInfo结构体依赖关系
     */
    async fn bloginfo_handle(list: &mut Vec<BlogInfo>) {
        for item in list.iter_mut() {
            let id = item.id.unwrap();
            item.category = Some(CategoryDao::get_by_bloginfo_id(id).await.unwrap());
            if let Some(password) = &item.password {
                //如果password!=null
                if *password != "" {
                    item.privacy = Some(true);
                }
            } else {
                item.privacy = Some(false)
            }
            item.password = Some(String::from(""));
            //转HTML
            item.description = markdown::to_html(&item.description);
            let tags = TagDao::get_blog_tags(id).await;
            item.tags = Some(tags);
            item.create_time = item.create_time.as_str()[0..19].to_string();
        }
    }

    /**
     * 所有文章总数(包含隐藏) -后台
     */
    pub async fn get_blog_count() -> i32 {
        BlogDao::get_blog_count().await.unwrap_or_default()
    }

    /**
     * 获取所有文章，用于首页展示，每页10条数据，并返回总页数，用于分页展示。 -后台
     */
    pub async fn get_blog_all_page(mut page: SearchRequest) -> ValueMap {
        if page.get_title() == "" {
            let _ = &page.set_title(None);
        }
        let mut map: ValueMap = ValueMap::new();
        let mut page_list = BlogDao::select_page_blog_dto(&page)
            .await
            .unwrap_or_else(|opt| {
                log::error!("{:?}", opt);
                Page::new(0, 0)
            });
        for item in page_list.get_records_mut() {
            if item.get_password().is_none() {
                item.set_password(Some(""));
            }
            item.set_category(Some(
                CategoryDao::get_by_id(item.get_category_id())
                    .await
                    .unwrap(),
            ));
        }
        map.insert(to_value!("pageNum"), to_value!(page_list.page_no()));
        map.insert(to_value!("pageNum"), to_value!(page_list.page_no()));
        map.insert(to_value!("pageSize"), to_value!(page_list.page_size()));
        map.insert(to_value!("pages"), to_value!(page_list.pages()));
        map.insert(to_value!("total"), to_value!(page_list.total()));
        map.insert(to_value!("list"), to_value!(page_list.get_records()));
        map
    }

    //根据ID查找博文 - 后台
    pub async fn update_visibility(v: &BlogVisibility) -> bool {
        let ok = BlogVisibility::update_by_column(&RBATIS.acquire().await.unwrap(), v, "id")
            .await
            .is_ok();
        ok
    }
    /**
     * 获取id的文章 -后台
     */
    pub(crate) async fn get_blog_dto(id: u16) -> Option<BlogDto> {
        let list =
            BlogDto::get_blog(&RBATIS.acquire().await.unwrap(), id.to_string().as_str()).await;

        match list {
            Ok(mut blogs) => {
                if !blogs.is_empty() {
                    let mut blog = blogs.pop().expect("异常");
                    //分类
                    blog.set_category(Some(
                        CategoryDao::get_by_id(blog.get_category_id())
                            .await
                            .unwrap(),
                    ));
                    //todo 标签
                    let mut tags = vec![];
                    let tags_id = TagService::get_tags_by_blog_id(blog.get_id()).await;
                    for id in tags_id {
                        let tag = TagService::get_tags_by_id(id).await;
                        tags.push(tag)
                    }
                    blog.set_tags(Some(tags));
                    return Some(blog);
                }
                None
            }

            Err(e) => {
                log::error!("{:?}", e);
                None
            }
        }
    }
    /**
     * 添加或者更新文章
     */
    pub(crate) async fn update_blog_dto(mut query: BlogVO) -> bool {
        // 开启事务
        let mut tx = match RBATIS.acquire_begin().await {
            Ok(tx) => tx,
            Err(e) => {
                log::error!("开启事务失败: {:?}", e);
                return false;
            }
        };

        // 新增或更新文章
        if query.get_id() == 0 {
            query.set_create_time(DateTime::now());
            query.set_update_time(DateTime::now());
            let data = match BlogVO::insert(&tx, &query).await {
                Ok(data) => data,
                Err(e) => {
                    log::error!("插入文章失败: {:?}", e);
                    tx.rollback()
                        .await
                        .unwrap_or_else(|e| log::error!("回滚事务失败: {:?}", e));
                    return false;
                }
            };
            // 获取新插入的id
            let new_id = match data.last_insert_id {
                Value::U64(id) => id as u16,
                Value::U32(id) => id as u16,
                _ => {
                    log::error!("获取新插入的id失败");
                    tx.rollback()
                        .await
                        .unwrap_or_else(|e| log::error!("回滚事务失败: {:?}", e));
                    return false;
                }
            };

            // 更新添加标签
            let add_tags = query.get_tag_list().unwrap_or_default();
            if !BlogTagService::inser_tags(add_tags, new_id, &mut tx).await {
                tx.rollback()
                    .await
                    .unwrap_or_else(|e| log::error!("回滚事务失败: {:?}", e));
                return false;
            }
        } else {
            query.set_update_time(DateTime::now());
            if !BlogVO::update_by_column(&tx, &query, "id").await.is_ok() {
                log::error!("更新文章失败");
                tx.rollback()
                    .await
                    .unwrap_or_else(|e| log::error!("回滚事务失败: {:?}", e));
                return false;
            }

            // 标签处理
            let add_tags = query.get_tag_list().unwrap_or_default();
            let remove_tags = TagService::get_tags_by_blog_id(query.get_id()).await;
            let (add, remove) = BlogService::array_diff(add_tags, remove_tags);

            if !BlogTagService::inser_tags(add, query.get_id(), &mut tx).await {
                tx.rollback()
                    .await
                    .unwrap_or_else(|e| log::error!("回滚事务失败: {:?}", e));
                return false;
            }

            if !BlogTagService::remove_tags(remove, query.get_id(), &mut tx).await {
                tx.rollback()
                    .await
                    .unwrap_or_else(|e| log::error!("回滚事务失败: {:?}", e));
                return false;
            }
        }

        // 提交事务
        if let Err(e) = tx.commit().await {
            log::error!("提交事务失败: {:?}", e);
            tx.rollback()
                .await
                .unwrap_or_else(|e| log::error!("回滚事务失败: {:?}", e));
            return false;
        }

        true
    }

    //比对数组差异并返回
    pub fn array_diff(arr1: Vec<u16>, arr2: Vec<u16>) -> (Vec<u16>, Vec<u16>) {
        let mut add_result = Vec::new();
        let mut delete_result = Vec::new();
        //新数据 剔除重复，存在即新增
        for item in &arr1 {
            if !arr2.contains(item) {
                add_result.push(item.clone());
            }
        }
        //旧数据 剔除重复，存在即删除
        for item in &arr2 {
            if !arr1.contains(item) {
                delete_result.push(item.clone());
            }
        }
        (add_result, delete_result)
    }
}

#[cfg(test)]
mod tests {
    use rbatis::rbdc::DateTime;

    use crate::service::BlogService;

    #[test]
    pub(crate) fn test_datetime() {
        let time = DateTime::now().format("YYYY-MM-DD hh:mm:ss");
        println!("{:?}", time)
        //stdout "2024-05-18 12:46:22"
    }

    //毕竟两个数组的相同数并剔除重复
    #[test]
    pub(crate) fn test_array_diff() {
        let add = vec![1, 2, 3, 4, 5];
        let remove = vec![2, 3, 4, 5];
        let (add, delete) = BlogService::array_diff(add, remove);
        println!("add : {:?}, delete : {:?}", add, delete);
        //assert_eq!(add, vec![]);
        //assert_eq!(delete, vec![]);
    }

    //stdout add : [6], delete : []
    //stdout ["a", "d", "e", "f"]
    //stdout add : [], delete : [1, 2, 3, 4, 5, 6]
}
