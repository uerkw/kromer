pub use sea_orm_migration::prelude::*;

mod m20240113_000001_create_address;
mod m20240113_000001_create_transaction;
mod m20240113_000001_create_name;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240113_000001_create_address::Migration),
            Box::new(m20240113_000001_create_transaction::Migration),
            Box::new(m20240113_000001_create_name::Migration),
            ]
    }
}
