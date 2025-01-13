use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

impl Migration {
    async fn create_transaction_indexes(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("transaction_from")
                    .table(Transaction::Table)
                    .col(Transaction::From)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("transaction_to")
                    .table(Transaction::Table)
                    .col(Transaction::To)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("transaction_name")
                    .table(Transaction::Table)
                    .col(Transaction::Name)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("transaction_sent_name")
                    .table(Transaction::Table)
                    .col(Transaction::SentName)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("transaction_sent_metaname")
                    .table(Transaction::Table)
                    .col(Transaction::SentMetaname)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("transaction_sent_metaname_sent_name")
                    .table(Transaction::Table)
                    .col(Transaction::SentMetaname)
                    .col(Transaction::SentName)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("transaction_request_id")
                    .table(Transaction::Table)
                    .col(Transaction::RequestId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("transaction_metadata")
                    .table(Transaction::Table)
                    .col(Transaction::Metadata)
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
                    .table(Transaction::Table)
                    .if_not_exists()
                    .col(pk_auto(Transaction::Id))
                    .col(ColumnDef::new(Transaction::From).string_len(10).null())
                    .col(ColumnDef::new(Transaction::To).string_len(10).null())
                    .col(float(Transaction::Value))
                    .col(timestamp(Transaction::Time).timestamp_with_time_zone())
                    .col(ColumnDef::new(Transaction::Name).string_len(128).null())
                    .col(
                        ColumnDef::new(Transaction::SentMetaname)
                            .string_len(32)
                            .null(),
                    )
                    .col(ColumnDef::new(Transaction::SentName).string_len(64).null())
                    .col(
                        ColumnDef::new(Transaction::Metadata)
                            .string_len(512)
                            .null(),
                    )
                    .col(uuid(Transaction::RequestId).unique_key())
                    .to_owned(),
            )
            .await?;

        Migration::create_transaction_indexes(manager).await
    
    }
    
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Transaction::Table).to_owned())
            .await
    }
}


#[derive(DeriveIden)]
enum Transaction {
    Table,
    Id,
    From,
    To,
    Value,
    Time,
    Name,
    SentMetaname,
    SentName,
    RequestId,
    Metadata, // Called `op` in Krist.
}
