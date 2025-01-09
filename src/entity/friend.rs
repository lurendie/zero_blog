//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;

use crate::model::Friend;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "friend")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub nickname: String,
    pub description: String,
    pub website: String,
    pub avatar: String,
    pub is_published: bool,
    pub views: i32,
    pub create_time: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl From<Friend> for Model {
    fn from(friend: Friend) -> Self {
        Self {
            id: friend.id.unwrap_or_default(),
            nickname: friend.nickname,
            description: friend.description,
            website: friend.website,
            avatar: friend.avatar,
            is_published: friend.is_published,
            views: friend.views,
            create_time: friend.create_time,
        }
    }
}
