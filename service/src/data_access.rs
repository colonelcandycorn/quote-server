use super::data_transfer_objects::{AuthorDTO, QuoteCreateDTO, QuoteDTO, TagDTO};
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
    pub async fn get_tag_with_related_quotes(
        db: &DbConn,
        tag_id: i32,
        page: u64,
        page_size: u64,
    ) -> Result<Option<(TagDTO, Vec<QuoteDTO>, u64)>, DbErr> {
        let tag = Tag::find_by_id(tag_id).one(db).await?;

        if let Some(tag) = tag {
            let quotes_query = tag.find_related(Quote);

            let paginator = quotes_query.paginate(db, page_size);

            let total = paginator.num_pages().await?;

            let mut result: Vec<QuoteDTO> = Vec::new();

            for quote in paginator.fetch_page(page - 1).await? {
                let dto = Self::get_quote_with_related_tags_and_author(db, quote).await?;

                result.push(dto);
            }

            if result.is_empty() {
                return Ok(None);
            }

            return Ok(Some((tag.into(), result, total)));
        }

        Ok(None)
    }

    pub async fn get_author_with_related_quotes(
        db: &DbConn,
        author_id: i32,
        page: u64,
        page_size: u64,
    ) -> Result<Option<(AuthorDTO, Vec<QuoteDTO>, u64)>, DbErr> {
        let author = Author::find_by_id(author_id).one(db).await?;

        if let Some(author) = author {
            let quotes_query = author.find_related(Quote);

            let paginator = quotes_query.paginate(db, page_size);

            let total = paginator.num_pages().await?;

            let mut result: Vec<QuoteDTO> = Vec::new();

            for quote in paginator.fetch_page(page - 1).await? {
                let dto = Self::get_quote_with_related_tags_and_author(db, quote).await?;

                result.push(dto);
            }

            if result.is_empty() {
                return Ok(None);
            }

            return Ok(Some((author.into(), result, total)));
        }

        Ok(None)
    }

    pub async fn get_author(db: &DbConn, author_id: i32) -> Result<Option<AuthorDTO>, DbErr> {
        let result = Author::find_by_id(author_id).one(db).await?;

        if let Some(result) = result {
            return Ok(Some(result.into()));
        }

        Ok(None)
    }

    pub async fn get_or_create_author_model(
        db: &DbConn,
        author_name: String,
    ) -> Result<AuthorDTO, DbErr> {
        let author_name_lower = author_name.to_lowercase();

        let author_results = Author::find()
            .filter(author::Column::Name.eq(&author_name_lower))
            .all(db)
            .await;

        
        if let Ok(authors) = author_results {
            if let Some(author) = authors.into_iter().next() {
                let dto: AuthorDTO = author.into();
                return Ok(dto);
            }
        }

        tracing::info!(
            "Author with name '{}' not found, creating new author.",
            author_name_lower
        );
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

            let _quote_tag_association =
                DataAccess::create_quote_tag_association(db, &quote_model, &tag_dto).await?;

            related_tags.push(tag_dto);
        }

        let dto = QuoteDTO {
            id: quote_model.id,
            quote: quote_model.quote,
            related_tags,
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
    ) -> Result<Option<(Vec<QuoteDTO>, u64)>, DbErr> {
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

        if result.is_empty() {
            return Ok(None);
        }

        Ok(Some((result, total)))
    }

    pub async fn get_tags_in_page(
        db: &DbConn,
        page: u64,
        page_size: u64,
    ) -> Result<Option<(Vec<TagDTO>, u64)>, DbErr> {
        let query = Tag::find().order_by(tag::Column::Tag, Order::Asc);

        let paginator = query.paginate(db, page_size);
        let total = paginator.num_pages().await?;

        let mut result: Vec<TagDTO> = Vec::new();

        for tag in paginator.fetch_page(page - 1).await? {
            result.push(tag.into());
        }

        if result.is_empty() {
            return Ok(None);
        }

        Ok(Some((result, total)))
    }

    pub async fn get_authors_in_page(
        db: &DbConn,
        page: u64,
        page_size: u64,
    ) -> Result<Option<(Vec<AuthorDTO>, u64)>, DbErr> {
        let query = Author::find().order_by(author::Column::Name, Order::Asc);

        let paginator = query.paginate(db, page_size);
        let total = paginator.num_pages().await?;

        let mut result: Vec<AuthorDTO> = Vec::new();

        for author in paginator.fetch_page(page - 1).await? {
            result.push(author.into());
        }

        if result.is_empty() {
            return Ok(None);
        }

        Ok(Some((result, total)))
    }

    // TAGS
    pub async fn get_tag_or_create_tag(db: &DbConn, tag: String) -> Result<TagDTO, DbErr> {
        let tag_lower = tag.to_lowercase();
        let search_results = DataAccess::get_tags(db, &tag_lower).await;

        if let Ok(search_results) = search_results {
            if let Some(tag) = search_results.into_iter().next() {
                return Ok(tag.into());
            }
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
        tracing::info!("Creating tag: {}", tag);
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
    ) -> Result<quote_tag_association::Model, DbErr> {

        tracing::info!(
            "\n\n\tCreating quote-tag association for quote_id: {}, tag_id: {}",
            quote.id, tag.id
        );

        match (quote_tag_association::ActiveModel {
            quote_id: Set(quote.id),
            tag_id: Set(tag.id),
        }
        .insert(db)
        .await)
        {
            Ok(am) => Ok(am),
            Err(e) => {
                tracing::error!("Failed to save quote_tag_association: {:?}", e);
                Err(e)
            }
        }
    }
}
