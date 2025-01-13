use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

impl Migration {
    async fn create_name_indexes(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("name_name_unique")
                    .table(Name::Table)
                    .col(Name::Name)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("name_owner")
                    .table(Name::Table)
                    .col(Name::Owner)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("name_original_owner")
                    .table(Name::Table)
                    .col(Name::OriginalOwner)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("name_unpaid")
                    .table(Name::Table)
                    .col(Name::Unpaid)
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
                    .table(Name::Table)
                    .if_not_exists()
                    .col(pk_auto(Name::Id))
                    .col(
                        ColumnDef::new(Name::Name)
                            .string_len(64)
                            .unique_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Name::Owner).char_len(10).not_null())
                    .col(ColumnDef::new(Name::OriginalOwner).char_len(10).null())
                    .col(timestamp(Name::Registered).timestamp())
                    .col(
                        ColumnDef::new(Name::Updated)
                            .timestamp()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Name::Transferred)
                            .timestamp()
                            .null(),
                    )
                    .col(ColumnDef::new(Name::Metadata).string_len(255).null())
                    .col(float(Name::Unpaid))
                    .to_owned(),
            )
            .await?;

        Migration::create_name_indexes(manager).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Name::Table).to_owned())
            .await
    }
}


#[derive(DeriveIden)]
enum Name {
    Table,
    Id,
    Name,
    Owner,
    OriginalOwner,
    Registered,
    Updated,
    Transferred,
    Metadata, // Called `a` in Krist.
    Unpaid,
}
