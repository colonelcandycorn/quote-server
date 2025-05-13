use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QuoteCreateDTO {
    pub quote: String,
    pub related_tags: Vec<TagCreateDTO>,
    pub author_name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QuoteDTO {
    pub id: i32,
    pub quote: String,
    pub related_tags: Vec<TagDTO>,
    pub author: AuthorDTO,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TagCreateDTO {
    pub tag: String,
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

#[derive(Clone, Debug, Serialize, Deserialize)]
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
