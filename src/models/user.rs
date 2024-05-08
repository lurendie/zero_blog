use rbatis::impl_select;
use serde::{Deserialize, Serialize};

/*
 * @Author: lurendie 549700459@qq.com
 * @Date: 2024-02-24 22:58:03
 * @LastEditors: lurendie
 * @LastEditTime: 2024-05-06 23:42:14
 */
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub(crate) struct User {
    id: u16,
    username: String,    //用户名
    password: String,    //密码
    nickname: String,    //昵称
    avatar: String,      //头像
    email: String,       //邮箱
    create_time: String, //创建时间
    update_time: String, //更新时间
    role: String,        //角色访问权限
}

impl User {
    pub fn get_id(&self) -> i32 {
        self.id as i32
    }

    pub fn get_username(&self) -> String {
        self.username.clone()
    }

    pub fn get_password(&self) -> String {
        self.password.clone()
    }

    pub fn set_password(&mut self, pass: String) {
        self.password = pass
    }
}

impl_select!(User {});
