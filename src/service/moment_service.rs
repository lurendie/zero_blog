use crate::models::dto::moment_dto::MomentDTO;
use crate::rbatis::get_conn;
use crate::utils::MarkdownParser;
use crate::{dao::MomentDao, models::moment::Moment};
use rbatis::Page;
use rbatis::{rbdc::datetime::DateTime, IPage};
pub struct MomentService;

impl MomentService {
    //获取所有的动态
    pub(crate) async fn get_moments(page_num: usize) -> Page<Moment> {
        let moments = MomentDao::get_moments(page_num).await.unwrap_or_else(|e| {
            log::error!("{}", e);
            //出现异常则返回初始化对象
            Page::new(0, 0, 0, vec![])
        });
        moments
    }
    //创建动态
    pub async fn create_and_update_moment(
        mut moment: MomentDTO,
    ) -> Result<u64, rbatis::rbdc::Error> {
        let tx = get_conn().await;
        let row;
        if moment.get_id().unwrap_or(0) > 0 {
            row = MomentDTO::update_by_column(&tx, &moment, "id").await?;
        } else {
            moment.set_create_time(DateTime::now().to_string());
            row = MomentDTO::insert(&tx, &moment).await?;
        }

        Ok(row.rows_affected)
    }

    //获取公开的动态
    pub(crate) async fn get_public_moments(page_num: usize) -> Page<Moment> {
        let mut moments = MomentDao::get_public_moments(page_num)
            .await
            .unwrap_or_else(|e| {
                log::error!("{}", e);
                //出现异常则返回初始化对象
                Page::new(0, 0, 0, vec![])
            });
        moments
            .records_mut()
            .iter_mut()
            .for_each(|item: &mut Moment| {
                item.create_time = item.create_time.as_str()[0..19].to_string();
                item.content = MarkdownParser::parser_html(&item.content);
            });
        moments
    }

    /**
     * 更新动态的发布状态
     */
    pub(crate) async fn update_published(
        id: u16,
        is_published: bool,
    ) -> Result<u64, rbatis::rbdc::Error> {
        let tx = get_conn().await;
        let mut table = MomentDTO::default();
        table.set_id(id as u16);
        table.set_is_published(is_published);
        let row = MomentDTO::update_by_column(&tx, &table, "id").await?;
        Ok(row.rows_affected)
    }

    /**
     * 删除动态
     */
    pub(crate) async fn delete_moment(id: u16) -> Result<u64, rbatis::rbdc::Error> {
        let tx = get_conn().await;
        let row = MomentDTO::delete_by_column(&tx, "id", id).await?;
        Ok(row.rows_affected)
    }

    /**
     * 获取ID动态
     */

    pub(crate) async fn get_moment_by_id(id: u16) -> Option<Moment> {
        let tx = get_conn().await;
        let mut moments = Moment::select_by_column(&tx, "id", id)
            .await
            .unwrap_or_else(|e| {
                log::error!("get_moment_by_id error:{}", e);
                //出现异常则返回初始化对象
                vec![]
            });
        if moments.len() > 0 {
            let moment = moments.pop()?;
            return Some(moment);
        }
        None
    }

    pub async fn moment_like(id: u16) -> Result<u64, rbatis::rbdc::Error> {
        let executor = get_conn().await;
        let mut table = Moment::select_by_column(&executor, "id", id.to_string().as_str()).await?;
        for item in table.iter_mut() {
            item.likes += 1;
            let query = MomentDao::update_likes(item.id.unwrap_or_default(), item.likes).await?;
            return Ok(query);
        }
        Ok(0)
    }
}
