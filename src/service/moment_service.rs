use crate::models::dto::moment_dto::MomentDTO;
use crate::rbatis::{RBATIS,get_conn};
use crate::utils::MarkdownParser;
use crate::{dao::MomentDao, models::moment::Moment};
use rbatis::rbatis_codegen::ops::AsProxy;
use rbatis::{IPage, Page};
pub struct MomentService;

impl MomentService {
    //获取所有的动态
    pub(crate) async fn get_moments(page_num: usize) -> Page<Moment> {
        let moments = MomentDao::get_moments(page_num).await.unwrap_or_else(|e| {
            log::error!("{}", e);
            //出现异常则返回初始化对象
            Page::new(0, 0)
        });
        moments
    }
    //创建动态
    pub async fn create_moment(moment: MomentDTO) -> Result<u64, rbatis::rbdc::Error> {
        let tx = RBATIS.acquire().await.expect("get tx error");
        let row = MomentDTO::insert(&tx, &moment).await?;

        Ok(row.rows_affected)
    }

    //获取公开的动态
    pub(crate) async fn get_public_moments(page_num: usize) -> Page<Moment> {
        let mut moments = MomentDao::get_public_moments(page_num).await.unwrap_or_else(|e| {
            log::error!("{}", e);
            //出现异常则返回初始化对象
            Page::new(0, 0)
        });
        moments
            .get_records_mut()
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
    pub(crate) async fn update_published(id: u16, is_published: bool) -> Result<u64, rbatis::rbdc::Error> {
        let tx = get_conn().await;
        let mut table=MomentDTO::default();
        table.set_id(id as u16);
        table.set_is_published(is_published.u32() as u8);
       let row = MomentDTO::update_by_column(&tx, &table, "id").await?;
       Ok(row.rows_affected)
    }
}
