use chrono::NaiveDateTime;
use rocket::serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;

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
    pub id: i64,
    pub uid: String,
    pub timestamp: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub lat: f64,
    pub lng: f64,
    pub accuracy: i16,
    pub activity: Activity,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GeoDataJson {
    pub timestamp: NaiveDateTime,
    pub lat: f64,
    pub lng: f64,
    pub accuracy: i16,
    pub activity: Activity,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl From<Model> for GeoDataJson {
    fn from(input: Model) -> Self {
        Self {
            timestamp: input.timestamp,
            lat: input.lat,
            lng: input.lng,
            accuracy: input.accuracy,
            activity: input.activity,
        }
    }
}
