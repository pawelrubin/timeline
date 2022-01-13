use chrono::NaiveDateTime;
use rocket::serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;


#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[sea_orm(table_name = "path")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
}


#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::geodata::Entity")]
    Geodata,
}

impl Related<super::GeodataEntity> for Entity {
    fn to() -> RelationDef {
        Relation::Geodata.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

