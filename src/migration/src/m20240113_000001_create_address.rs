use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

impl Migration {
    async fn create_address_indexes(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("address_address_unique")
                    .table(Address::Table)
                    .col(Address::Address)
                    .unique()
                    .to_owned(),
            )
            .await
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Define how to apply this migration
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Address::Table)
                    .if_not_exists()
                    .col(pk_auto(Address::Id))
                    .col(char_len(Address::Address, 10).unique_key().not_null())
                    .col(float(Address::Balance))
                    .col(float(Address::TotalIn))
                    .col(float(Address::TotalOut))
                    .col(timestamp(Address::FirstSeen).timestamp())
                    .col(
                        ColumnDef::new(Address::PrivateKey)
                            .string_len(64)
                            .null()
                            .take(),
                    )
                    .col(
                        ColumnDef::new(Address::Alert)
                            .string_len(1024)
                            .null()
                            .take(),
                    )
                    .col(boolean(Address::Locked).default(false))
                    .to_owned(),                    
            )
            .await?;
    
    Migration::create_address_indexes(manager).await
    
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Address::Table).to_owned())
            .await
    }
}


#[derive(DeriveIden)]
enum Address {
    Table,
    Id,
    Address,
    Balance,
    TotalIn,
    TotalOut,
    FirstSeen,
    PrivateKey,
    Alert,
    Locked,
}
