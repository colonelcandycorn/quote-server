//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.10

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "tag")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub tag: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::quote_tag_association::Entity")]
    QuoteTagAssociation,
}

impl Related<super::quote_tag_association::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::QuoteTagAssociation.def()
    }
}

impl Related<super::quote::Entity> for Entity {
    fn to() -> RelationDef {
        super::quote_tag_association::Relation::Quote.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::quote_tag_association::Relation::Tag.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
