use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QuoteDTO {
    pub name: String,
    pub quote: String,
    pub related_tags: Vec<TagDTO>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
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
