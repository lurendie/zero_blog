use crate::models::moment::Moment;
use crate::rbatis::RBATIS;
use rbatis::Error;
use rbatis::Page;
use rbatis::PageRequest;
use rbs::to_value;
pub struct MomentDao;
impl MomentDao {
    //获取的动态(不包括隐藏动态)
    pub(crate) async fn get_moments(page_num: usize) -> Result<Page<Moment>, Error> {
        let moments = Moment::select_page(
            &RBATIS.acquire().await.unwrap(),
            &PageRequest::new(page_num as u64, 5),
        )
        .await;
        moments
    }
//获取的动态(包含隐藏动态)
    pub async fn get_public_moments(page_num: usize)->Result<Page<Moment>, Error>{
        let moments = Moment::select_published_page(
            &RBATIS.acquire().await.unwrap(),
            &PageRequest::new(page_num as u64, 10),
        )
        .await;
        moments
    }

    pub async fn update_likes(id: u64, likes: u64) ->  Result<u64, Error>{
        let sql = "UPDATE moment SET likes = ? WHERE id = ?";
        let args = vec![to_value!(likes), to_value!(id)];
       let row= RBATIS.exec(sql, args).await?;
        Ok(row.rows_affected)
    }
}
