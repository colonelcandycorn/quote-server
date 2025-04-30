pub use sea_orm_migration::prelude::*;

mod m20240424_000001_create_quote_table;
mod m20250430_133655_create_tags_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240424_000001_create_quote_table::Migration),
            Box::new(m20250430_133655_create_tags_table::Migration),
        ]
    }
}
