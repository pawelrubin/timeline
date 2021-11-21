use sea_orm::entity::prelude::*;
use chrono::NaiveDateTime;
use rocket::serde::{Serialize,Deserialize,uuid::Uuid};

#[derive(Clone, Debug, EnumIter, PartialEq, DeriveActiveEnum, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "activity")]
pub enum Activity {
    #[sea_orm(string_value = "Still")]
    Still,
    #[sea_orm(string_value = "OnFoot")]
    OnFoot,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[sea_orm(table_name = "geodata")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub uid: i64,
    pub timestamp: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub lat: f64,
    pub lng: f64,
    pub accuracy: u8,
    pub activity: Activity
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}