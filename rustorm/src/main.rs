use rustorm::User;
use sqlx::PgPool;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let db = PgPool::connect(
        "postgres://postgres:admin@localhost/rustorm"
    )
    .await?;

    let users = User::find()
        .where_(User::name.eq("Fakhir"))
        .take(20)
        .all(&db)
        .await?;

    println!("{users:#?}");

    Ok(())
}