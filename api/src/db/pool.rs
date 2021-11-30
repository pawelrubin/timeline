use std::env;

use async_trait::async_trait;
use sea_orm_rocket::{rocket::figment::Figment, Database};

#[derive(Database, Debug)]
#[database("timeline")]
pub struct Db(SeaOrmPool);

#[derive(Debug, Clone)]
pub struct SeaOrmPool {
    pub conn: sea_orm::DatabaseConnection,
}

#[async_trait]
impl sea_orm_rocket::Pool for SeaOrmPool {
    type Error = sea_orm::DbErr;

    type Connection = sea_orm::DatabaseConnection;

    async fn init(_figment: &Figment) -> Result<Self, Self::Error> {
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let conn = sea_orm::Database::connect(&db_url).await?;
        Ok(SeaOrmPool { conn })
    }

    fn borrow(&self) -> &Self::Connection {
        &self.conn
    }
}
