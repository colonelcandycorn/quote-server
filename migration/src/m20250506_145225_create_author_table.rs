use sea_orm_migration::{prelude::*, schema::*, sea_orm::{DbBackend, Statement}};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(Author::Table)
                    .if_not_exists()
                    .col(pk_auto(Author::Id))
                    .col(string(Author::Name))
                    .to_owned()
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Quote::Table)
                    .add_column(integer(Quote::AuthorId).not_null().default(0))
                    .to_owned(),
            )
            .await?;

        let db = manager.get_connection();

        db
            .execute(
                Statement::from_string(
                    DbBackend::Sqlite,
                    r#"
                    INSERT INTO author (name)
                    SELECT DISTINCT name FROM quote
                    "#
                    .to_owned(),
                )
            )
            .await?;

        db
            .execute(
                Statement::from_string(
                    DbBackend::Sqlite,
                    r#"
                    UPDATE quote
                    SET author_id = author_tbl.id
                    FROM (SELECT id, name FROM author) as author_tbl
                    WHERE quote.name = author_tbl.name
                    "#
                    .to_owned(),
                )
            )
            .await?;

        manager
            .rename_table(
                Table::rename()
                    .table(Quote::Table, QuoteV1::Table)
                    .to_owned()
            )
            .await?;

        manager
            .rename_table(
                Table::rename()
                    .table(QuoteTagAssociation::Table, QuoteTagAssociationV1::Table)
                    .to_owned()
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Quote::Table)
                    .if_not_exists()
                    .col(pk_auto(Quote::Id))
                    .col(string(Quote::Quote))
                    .col(integer(Quote::AuthorId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-quote-author-id")
                            .from(Quote::Table, Quote::AuthorId)
                            .to(Author::Table, Author::Id),
                    )
                    .to_owned()
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(QuoteTagAssociation::Table)
                    .if_not_exists()
                    .col(integer(QuoteTagAssociation::QuoteId).not_null())
                    .col(integer(QuoteTagAssociation::TagId).not_null())
                    // source: https://stackoverflow.com/questions/78101516/write-a-m-to-m-relation-in-sea-orm-migration
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
                    // source : https://stackoverflow.com/questions/78101516/write-a-m-to-m-relation-in-sea-orm-migration
                    .primary_key(
                        Index::create()
                            .col(QuoteTagAssociation::QuoteId)
                            .col(QuoteTagAssociation::TagId),
                    )
                    .to_owned(),
            )
            .await?;

        db
            .execute(
                Statement::from_string(
                    DbBackend::Sqlite,
                    r#"
                    INSERT INTO quote (author_id, quote)
                    SELECT author_id, quote FROM quote_v1
                    "#
                    .to_owned(),
                )
            )
            .await?;

        db
            .execute(
                Statement::from_string(
                    DbBackend::Sqlite,
                    r#"
                    INSERT INTO quote_tag_association (quote_id, tag_id)
                    SELECT quote_id, tag_id from quote_tag_association_v1
                    "#
                    .to_owned(),
                )
            )
            .await?;

        manager
            .drop_table(Table::drop().table(QuoteV1::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(QuoteTagAssociationV1::Table).to_owned())
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                .table(Quote::Table)
                .add_column(string(Quote::Name).default(""))
                .to_owned()
            )
            .await?;

        let db = manager.get_connection();

        db.execute(
            Statement::from_string(
                DbBackend::Sqlite,
                r#"
                UPDATE quote
                SET name = (
                    SELECT author.name
                    FROM author
                    WHERE author.id = quote.author_id
                );
                "#
                .to_owned()
            )
        )
        .await?;

        manager
            .rename_table(
                Table::rename()
                    .table(Quote::Table, QuoteV1::Table)
                    .to_owned()
            )
            .await?;

        manager
            .create_table(
                sea_query::Table::create()
                    .table(Quote::Table)
                    .if_not_exists()
                    .col(pk_auto(Quote::Id))
                    .col(string(Quote::Name))
                    .col(string(Quote::Quote))
                    .to_owned(),
            )
            .await?;

        db
            .execute(
                Statement::from_string(
                    DbBackend::Sqlite,
                    r#"
                    INSERT INTO quote (name, quote)
                    SELECT name, quote FROM quote_v1
                    "#
                    .to_owned(),
                )
            )
            .await?;

        manager
            .drop_table(Table::drop().table(QuoteV1::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Author::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Author {
    Table,
    Id,
    Name,
}

#[derive(DeriveIden)]
pub enum Quote {
    Table,
    Id,
    Name,
    AuthorId,
    Quote,
}

#[derive(DeriveIden)]
pub enum QuoteV1 {
    Table,
}

#[derive(DeriveIden)]
enum Tag {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum QuoteTagAssociation {
    Table,
    TagId,
    QuoteId,
}

#[derive(DeriveIden)]
enum QuoteTagAssociationV1 {
    Table,
}