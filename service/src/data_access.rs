use super::data_transfer_objects::{AuthorDTO, QuoteCreateDTO, QuoteDTO, TagCreateDTO, TagDTO};
use ::entity::{
    author::{self, Entity as Author},
    quote::{self, Entity as Quote},
    quote_tag_association,
    tag::{self, Entity as Tag},
};
use sea_orm::*;

pub struct DataAccess {}

/*
source: https://github.com/SeaQL/sea-orm/blob/master/examples/axum_example/service/src/mutation.rs

and

source: https://github.com/SeaQL/sea-orm/blob/master/examples/axum_example/service/src/query.rs
*/
impl DataAccess {
    // AUTHOR
    // id
    // name

    pub async fn get_or_create_author_model(
        db: &DbConn,
        author_name: String,
    ) -> Result<AuthorDTO, DbErr> {
        let author_name_lower = author_name.to_lowercase();

        let author_results: Vec<author::Model> = Author::find()
            .filter(author::Column::Name.eq(&author_name_lower))
            .all(db)
            .await?;

        if author_results.len() > 1 {
            // some kind of error or logging?
        }

        if let Some(author) = author_results.into_iter().next() {
            let dto: AuthorDTO = author.into();
            return Ok(dto);
        }

        // doesn't exist so we need to create
        Ok(author::ActiveModel {
            name: Set(author_name_lower),
            ..Default::default()
        }
        .insert(db)
        .await?
        .into())
    }

    // QUOTE
    // id
    // quote
    // author_id

    pub async fn create_quote(db: &DbConn, quote: QuoteCreateDTO) -> Result<QuoteDTO, DbErr> {
        let author_dto = DataAccess::get_or_create_author_model(db, quote.author_name).await?;

        let quote_model = quote::ActiveModel {
            author_id: Set(author_dto.id),
            quote: Set(quote.quote.to_owned()),
            ..Default::default()
        }
        .insert(db)
        .await?;

        let mut related_tags: Vec<TagDTO> = Vec::new();

        for tag in quote.related_tags {
            let tag_dto = DataAccess::get_tag_or_create_tag(db, tag.tag).await?;

            let quote_tag_association =
                DataAccess::create_quote_tag_association(db, &quote_model, &tag_dto).await?;

            related_tags.push(tag_dto);
        }

        let dto = QuoteDTO {
            id: quote_model.id,
            quote: quote_model.quote,
            related_tags: related_tags,
            author: author_dto,
        };

        Ok(dto)
    }

    pub async fn get_quote(db: &DbConn, id: i32) -> Result<Option<quote::Model>, DbErr> {
        Quote::find_by_id(id).one(db).await
    }

    async fn get_quote_with_related_tags_and_author(
        db: &DbConn,
        quote: quote::Model,
    ) -> Result<QuoteDTO, DbErr> {
        let tags = quote.find_related(Tag).all(db).await?;
        let author = quote.find_related(Author).one(db).await?.unwrap(); // does it make sense for there to not be a related Author?

        Ok(QuoteDTO {
            id: quote.id,
            author: author.into(),
            quote: quote.quote,
            related_tags: tags.into_iter().map(TagDTO::from).collect(),
        })
    }

    pub async fn get_quotes_in_page(
        db: &DbConn,
        page: u64,
        page_size: u64,
    ) -> Result<(Vec<QuoteDTO>, u64), DbErr> {
        let query = Quote::find()
            .join(JoinType::LeftJoin, quote::Relation::Author.def())
            .order_by(author::Column::Name, Order::Asc);

        let paginator = query.paginate(db, page_size);
        let total = paginator.num_pages().await?;

        let mut result: Vec<QuoteDTO> = Vec::new();

        for quote in paginator.fetch_page(page - 1).await? {
            let dto = Self::get_quote_with_related_tags_and_author(db, quote).await?;

            result.push(dto);
        }

        Ok((result, total))
    }

    // TAGS
    pub async fn get_tag_or_create_tag(db: &DbConn, tag: String) -> Result<TagDTO, DbErr> {
        let tag_lower = tag.to_lowercase();
        let search_results = DataAccess::get_tags(db, &tag_lower).await?;

        if let Some(tag) = search_results.into_iter().next() {
            return Ok(tag.into());
        }

        let model = DataAccess::create_tag(db, &tag_lower).await?;

        Ok(model.into())
    }

    pub async fn get_tags(db: &DbConn, tag: &str) -> Result<Vec<tag::Model>, DbErr> {
        let tags = Tag::find()
            .filter(tag::Column::Tag.contains(tag))
            .all(db)
            .await?;

        Ok(tags)
    }

    pub async fn create_tag(db: &DbConn, tag: &str) -> Result<tag::Model, DbErr> {
        tag::ActiveModel {
            tag: Set(tag.to_owned()),
            ..Default::default()
        }
        .insert(db)
        .await
    }

    // QUOTE TAG ASSOCIATION

    pub async fn create_quote_tag_association(
        db: &DbConn,
        quote: &quote::Model,
        tag: &TagDTO,
    ) -> Result<quote_tag_association::ActiveModel, DbErr> {
        quote_tag_association::ActiveModel {
            quote_id: Set(quote.id),
            tag_id: Set(tag.id),
        }
        .save(db)
        .await
    }
}
