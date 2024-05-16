//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "kl_user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub role_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::kl_role::Entity",
        from = "Column::RoleId",
        to = "super::kl_role::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    KlRole,
}

impl Related<super::kl_role::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::KlRole.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
