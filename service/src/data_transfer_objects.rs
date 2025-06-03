use serde::{Deserialize, Deserializer, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct QuoteCreateDTO {
    pub quote: String,
    #[serde(deserialize_with = "deserialize_tags")]
    pub related_tags: Vec<TagCreateDTO>,
    pub author_name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct QuoteDTO {
    pub id: i32,
    pub quote: String,
    pub related_tags: Vec<TagDTO>,
    pub author: AuthorDTO,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct TagCreateDTO {
    pub tag: String,
}

// source: https://serde.rs/impl-deserialize.html
fn deserialize_tags<'de, D>(deserializer: D) -> Result<Vec<TagCreateDTO>, D::Error>
where
    D: Deserializer<'de>,
{
    let tags = Deserialize::deserialize(deserializer);

    Ok(tags.into_iter().map(|tag| TagCreateDTO { tag }).collect())
}

#[derive(Clone, Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct TagDTO {
    pub id: i32,
    pub tag: String,
}

impl From<entity::tag::Model> for TagDTO {
    fn from(item: entity::tag::Model) -> Self {
        TagDTO {
            id: item.id,
            tag: item.tag,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct AuthorDTO {
    pub id: i32,
    pub name: String,
}

impl From<entity::author::Model> for AuthorDTO {
    fn from(item: entity::author::Model) -> Self {
        AuthorDTO {
            id: item.id,
            name: item.name,
        }
    }
}
