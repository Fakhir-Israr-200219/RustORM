use rustorm::{Column, Entity, Field};
use sqlx::PgPool;
use sqlx::types::BigDecimal;

#[derive(Debug, sqlx::FromRow)]
pub struct UserModel {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, sqlx::FromRow)]
struct UserCount {
    #[allow(dead_code)]
    name: String,
    #[allow(dead_code)]
    count: i64,
}

#[derive(Debug, sqlx::FromRow)]
struct UserSum {
    #[allow(dead_code)]
    name: String,
    #[allow(dead_code)]
    sum: i64,
}

#[derive(Debug, sqlx::FromRow)]
struct UserAvg {
    #[allow(dead_code)]
    avg: BigDecimal,
}

pub struct User;

impl Entity for User {
    type Model = UserModel;
    const TABLE: &'static str = "users";
    const COLUMNS: &'static [Column] = &[Column::new("id"), Column::new("name")];
}

impl User {
    #[allow(non_upper_case_globals)]
    pub const id: Field<Self, i32> = Field::new("id");
    #[allow(non_upper_case_globals)]
    pub const name: Field<Self, String> = Field::new("name");
}

#[derive(Debug, sqlx::FromRow)]
pub struct PostModel {
    pub id: i32,
    pub title: String,
}

pub struct Post;

impl Entity for Post {
    type Model = PostModel;
    const TABLE: &'static str = "posts";
    const COLUMNS: &'static [Column] = &[Column::new("id"), Column::new("title")];
}

impl Post {
    #[allow(non_upper_case_globals)]
    pub const id: Field<Self, i32> = Field::new("id");
    #[allow(non_upper_case_globals)]
    pub const title: Field<Self, String> = Field::new("title");
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let db = PgPool::connect("postgres://postgres:admin@localhost/rustorm").await?;

    let users = User::find()
        .where_(User::name.eq("Fakhir"))
        .all(&db)
        .await?;

    let user_count = User::find().select(User::id.count()).count(&db).await?;

    let posts = Post::find().where_(Post::title.eq("Rust")).all(&db).await?;

    let user_count_with_where_ = User::find()
        .where_(User::name.eq("Fakhir"))
        .select(User::id.count())
        .count(&db)
        .await?;

    let grouped_counts = User::find()
        .select(User::name.select())
        .select(User::id.count())
        .group_by(User::name.column())
        .having(User::id.count().gt(1))
        .fetch_all::<UserCount>(&db)
        .await?;
    let user_id_sum = User::find().select(User::id.sum()).count(&db).await?;
    let user_id_sum_with_where = User::find()
        .where_(User::name.eq("Fakhir"))
        .select(User::id.sum())
        .count(&db)
        .await?;
    let grouped_sums = User::find()
        .select(User::name.select())
        .select(User::id.sum())
        .group_by(User::name.column())
        .having(User::id.sum().gt(2))
        .fetch_all::<UserSum>(&db)
        .await?;

    let user_avg = User::find()
        .select(User::id.avg())
        .fetch_all::<UserAvg>(&db)
        .await?;

    println!("User ID average: {:?}", user_avg);
    println!("Grouped sums: {grouped_sums:#?}");
    println!("User ID sum with where_: {user_id_sum_with_where}");
    println!("User ID sum: {user_id_sum}");
    println!("Users: {users:#?}");
    println!("User count: {user_count}");
    println!("Posts: {posts:#?}");
    println!("User Count with where_ clause: {user_count_with_where_:#?}");
    println!("Grouped counts: {grouped_counts:#?}");
    // User::find()
    // .where_(Post::title.eq("Rust"));

    Ok(())
}
