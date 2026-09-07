use rustorm::{Entity, Field};
use sqlx::PgPool;

#[derive(Debug, sqlx::FromRow)]
pub struct UserModel {
    pub id: i32,
    pub name: String,
}

pub struct User;

impl Entity for User {
    type Model = UserModel;

    const TABLE: &'static str = "users";

    fn columns() -> &'static [&'static str] {
        &["id", "name"]
    }
}

impl User {
    #[allow(non_upper_case_globals)]
    pub const id: Field<i32> = Field::new("id");

    #[allow(non_upper_case_globals)]
    pub const name: Field<String> = Field::new("name");
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

    fn columns() -> &'static [&'static str] {
        &["id", "title"]
    }
}

impl Post {
    #[allow(non_upper_case_globals)]
    pub const id: Field<i32> = Field::new("id");

    #[allow(non_upper_case_globals)]
    pub const title: Field<String> = Field::new("title");
}


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

    let posts = Post::find()
        .where_(Post::title.eq("Rust"))
        .take(10)
        .all(&db)
        .await?;

    println!("Users: {users:#?}");
    println!("Posts: {posts:#?}");

    Ok(())
}