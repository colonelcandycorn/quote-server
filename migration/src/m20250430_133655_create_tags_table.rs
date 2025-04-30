use super::m20240424_000001_create_quote_table::Quote;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Tag::Table)
                    .if_not_exists()
                    .col(pk_auto(Tag::Id))
                    .col(string(Tag::Tag))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(QuoteTagAssociation::Table)
                    .if_not_exists()
                    .col(integer(QuoteTagAssociation::QuoteId).not_null())
                    .col(integer(QuoteTagAssociation::TagId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-quote-tag-association-tag-id")
                            .from(QuoteTagAssociation::Table, QuoteTagAssociation::TagId)
                            .to(Tag::Table, Tag::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-quote-tag-association-quote-id")
                            .from(QuoteTagAssociation::Table, QuoteTagAssociation::QuoteId)
                            .to(Quote::Table, Quote::Id),
                    )
                    .primary_key(
                        Index::create()
                            .col(QuoteTagAssociation::QuoteId)
                            .col(QuoteTagAssociation::TagId),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Tag::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(QuoteTagAssociation::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Tag {
    Table,
    Id,
    Tag,
}

#[derive(DeriveIden)]
enum QuoteTagAssociation {
    Table,
    TagId,
    QuoteId,
}
