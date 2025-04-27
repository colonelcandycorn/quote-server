use ::entity::{quote, quote::Entity as Quote};
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

    pub async fn get_quotes_in_page(
        db: &DbConn,
        page: u64,
        page_size: u64,
    ) -> Result<(Vec<quote::Model>, u64), DbErr> {
        let paginator = Quote::find()
            .order_by_asc(quote::Column::Name)
            .paginate(db, page_size);
        let total = paginator.num_items().await?;

        paginator.fetch_page(page - 1).await.map(|items| (items, total))
    }
}