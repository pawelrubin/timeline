use crate::entities::*;
use sea_orm::{ConnectionTrait, DbConn, EntityTrait, Schema};

async fn create_enum<E>(db: &DbConn, entity: E)
where
    E: EntityTrait,
{
    let builder = db.get_database_backend();
    let schema = Schema::new(builder);
    let stmts = schema
        .create_enum_from_entity(entity)
        .iter()
        .map(|stmt| builder.build(stmt))
        .collect::<Vec<_>>();

    for stmt in stmts {
        match db.execute(stmt).await {
            Ok(_) => println!("Migrated {}", entity.table_name()),
            Err(e) => println!("Error: {}", e),
        }
    }
}

async fn create_table<E>(db: &DbConn, entity: E)
where
    E: EntityTrait,
{
    let builder = db.get_database_backend();
    let schema = Schema::new(builder);
    let stmt = builder.build(schema.create_table_from_entity(entity).if_not_exists());

    match db.execute(stmt).await {
        Ok(_) => println!("Migrated {}", entity.table_name()),
        Err(e) => println!("Error: {}", e),
    }
}

pub async fn create_tables(db: &DbConn) {
    create_enum(db, GeodataEntity).await;
    create_table(db, GeodataEntity).await;
}
