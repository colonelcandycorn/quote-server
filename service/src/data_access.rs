use super::data_transfer_objects::{QuoteDTO, TagDTO};
use ::entity::{quote, quote::Entity as Quote, tag::Entity as Tag};
use sea_orm::*;

pub struct DataAccess {}

/*
source: https://github.com/SeaQL/sea-orm/blob/master/examples/axum_example/service/src/mutation.rs

and

source: https://github.com/SeaQL/sea-orm/blob/master/examples/axum_example/service/src/query.rs
*/
impl DataAccess {
    pub async fn create_quote(
        db: &DbConn,
        quote: quote::Model,
    ) -> Result<quote::ActiveModel, DbErr> {
        quote::ActiveModel {
            name: Set(quote.name.to_owned()),
            quote: Set(quote.quote.to_owned()),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn get_quote(db: &DbConn, id: i32) -> Result<Option<quote::Model>, DbErr> {
        Quote::find_by_id(id).one(db).await
    }

    async fn get_quote_with_related_tags(
        db: &DbConn,
        quote: quote::Model,
    ) -> Result<QuoteDTO, DbErr> {
        let tags = quote.find_related(Tag).all(db).await?;

        Ok(QuoteDTO {
            name: quote.name,
            quote: quote.quote,
            related_tags: tags.into_iter().map(TagDTO::from).collect(),
        })
    }

    pub async fn get_quotes_in_page(
        db: &DbConn,
        page: u64,
        page_size: u64,
    ) -> Result<(Vec<QuoteDTO>, u64), DbErr> {
        let paginator = Quote::find()
            .order_by_asc(quote::Column::Name)
            .paginate(db, page_size);
        let total = paginator.num_pages().await?;

        let mut result: Vec<QuoteDTO> = Vec::new();

        for quote in paginator.fetch_page(page - 1).await? {
            let dto = Self::get_quote_with_related_tags(db, quote).await?;

            result.push(dto);
        }

        Ok((result, total))
    }
}
