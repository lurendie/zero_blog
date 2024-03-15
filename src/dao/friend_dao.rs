use crate::models::vo::friend_info::FriendInfo;
use rbatis::Error;
use crate::rbatis::RBATIS;


pub(crate) async fn get_friends()->Result<Vec<FriendInfo>,Error>{
   FriendInfo::select_all_published(&RBATIS.acquire().await.unwrap(), "1").await
}