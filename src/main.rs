use sea_orm::Database;
use sea_orm::DbErr;

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    let db = Database::connect("sqlite:temp.db").await?;

    Ok(())
}
