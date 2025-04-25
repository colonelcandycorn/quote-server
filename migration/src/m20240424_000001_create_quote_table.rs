use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

// impl MigrationName for Migration {
//     fn name(&self) -> &str {
//         "m20240424_000001_create_quote_table"
//     }
// }

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                sea_query::Table::create()
                    .table(Quote::Table)
                    .if_not_exists()
                    .col(pk_auto(Quote::Id))
                    .col(string(Quote::Name))
                    .col(string(Quote::Quote))
                    .to_owned()
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Quote::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Quote {
    Table,
    Id,
    Name,
    Quote,
}