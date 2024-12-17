use rbs::value::map::ValueMap;
use rbs::{to_value, Value};

use crate::constant::redis_key_constants;
use crate::dao::{BlogDao, BlogTagDao};

use crate::dao::TagDao;
use crate::models::dto::tag_dto::TagVO;
use crate::models::vo::serise::Series;
use crate::rbatis::RBATIS;

use super::RedisService;
pub struct TagService;
impl TagService {
    pub async fn get_tags() -> Vec<Value> {
        let mut result = vec![];
        //1.查询redis缓存
        let redis_cache =
            RedisService::get_value_vec(redis_key_constants::TAG_CLOUD_LIST.to_string()).await;
        if let Some(redis_cache) = redis_cache {
            let arr = match redis_cache {
                Value::Array(arr) => {
                    log::info!(
                        "reids KEY:{} 获取缓存数据成功",
                        redis_key_constants::TAG_CLOUD_LIST.to_string()
                    );
                    arr
                }
                _ => vec![],
            };
            return arr;
        }
        //2.查询数据库
        TagDao::get_list()
            .await
            .unwrap_or_default()
            .iter()
            .for_each(|item| result.push(to_value!(item)));

        //保存到Redis
        RedisService::set_value_vec(
            redis_key_constants::TAG_CLOUD_LIST.to_string(),
            &to_value!(&result),
        )
        .await;
        result
    }

    pub(crate) async fn get_tags_count() -> rbs::value::map::ValueMap {
        let mut map = ValueMap::new();
        let mut legend = vec![];
        let mut series = vec![];

        for item in TagDao::get_list().await.unwrap() {
            legend.push(to_value!(item.name.clone()));
            let series_item = Series::new(
                item.id.unwrap(),
                item.name.clone(),
                BlogDao::get_tags_count(item.name).await,
            );
            series.push(series_item);
        }
        map.insert(to_value!("legend"), to_value!(legend));
        map.insert(to_value!("series"), to_value!(series));
        map
    }

    pub(crate) async fn get_tags_by_blog_id(blog_id: u16) -> Vec<u16> {
        let mut list = Vec::new();
        for item in BlogTagDao::get_tags_by_blog_id(blog_id).await.unwrap() {
            list.push(item.get_tag_id());
        }
        list
    }

    pub(crate) async fn get_tags_by_id(tag_id: u16) -> TagVO {
        let tags = TagVO::select_by_column(
            &RBATIS.acquire().await.unwrap(),
            "id",
            tag_id.to_string().as_str(),
        )
        .await;
        let tag = match tags {
            Ok(mut list) => {
                if !list.is_empty() {
                    let tag = list.pop().unwrap();
                    return tag;
                }
                TagVO::default()
            }
            Err(err) => {
                log::error!("get_tags_by_blog_id err: {:?}", err);
                TagVO::default()
            }
        };
        tag
    }

    /**
     * 添加标签
     */
    pub async fn add_tag(name: String) -> Result<u16, rbatis::Error> {
        let mut tag = TagVO::default();
        tag.name = name;
        let result = TagVO::insert(&RBATIS.acquire().await.unwrap(), &tag).await;
        //添加标签
        match result {
            Ok(id) => match id.last_insert_id {
                Value::U32(id) => Ok(id as u16),
                _ => {
                    log::error!("获取新插入的id失败");
                    Ok(0)
                }
            },
            Err(err) => {
                log::error!("add_tag err: {:?}", err);
                Err(err)
            }
        }
    }
}
