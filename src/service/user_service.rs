/*
 * @Author: lurendie 
 * @Date: 2024-05-04 21:34:23
 * @LastEditors: lurendie
 * @LastEditTime: 2024-05-06 00:08:20
 */
/*
 * @Author: lurendie 
 * @Date: 2024-05-04 21:34:23
 * @LastEditors: lurendie
 * @LastEditTime: 2024-05-06 00:08:07
 */
/*
 * @Author: lurendie 
 * @Date: 2024-05-04 21:34:23
 * @LastEditors: lurendie
 * @LastEditTime: 2024-05-04 22:41:13
 * @FilePath: \zero_blog\src\service\user_service.rs
 */
use crate::dao::user_dao;
use crate::models::user::User;

/**
 *根据Name获取User
 */
pub async fn get_by_username(username: &String) -> Option<User> {
    let mut user_list = user_dao::get_by_username(&username).await;
    if let Ok(user_list) = user_list.as_mut() {
        if !user_list.is_empty() {
            let user = user_list.pop();
            return user;
        } else {
            return None;
        }
    }
    None
}
