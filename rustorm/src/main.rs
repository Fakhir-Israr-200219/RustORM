use rustorm::{
    entity::{Column, Entity, RelationKey, RelationLoader},
    field::Field,
    query::relation::Relation,
};

use sqlx::PgPool;
use sqlx::types::BigDecimal;

#[derive(Debug, sqlx::FromRow)]
pub struct UserModel {
    pub id: i32,
    pub name: String,

    #[sqlx(skip)]
    pub posts: Vec<PostModel>,
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

    #[allow(non_upper_case_globals)]
    pub const posts: Relation<Self, Post> = Relation::new(Self::id, Post::user_id);
}

impl RelationKey for UserModel {
    fn relation_key(&self, column: Column) -> Option<i64> {
        match column.name() {
            "id" => Some(self.id as i64),
            _ => None,
        }
    }
}

impl RelationLoader<PostModel> for UserModel {
    fn load_relation(&mut self, related: Vec<PostModel>) {
        self.posts = related;
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct PostModel {
    pub id: i32,
    pub user_id: i32,
    pub title: String,

    #[sqlx(skip)]
    pub comments: Vec<CommentModel>,
}

pub struct Post;

impl Entity for Post {
    type Model = PostModel;

    const TABLE: &'static str = "posts";

    const COLUMNS: &'static [Column] = &[
        Column::new("id"),
        Column::new("user_id"),
        Column::new("title"),
    ];
}

impl Post {
    #[allow(non_upper_case_globals)]
    pub const id: Field<Self, i32> = Field::new("id");

    #[allow(non_upper_case_globals)]
    pub const user_id: Field<Self, i32> = Field::new("user_id");

    #[allow(non_upper_case_globals)]
    pub const title: Field<Self, String> = Field::new("title");

    #[allow(non_upper_case_globals)]
    pub const comments: Relation<Self, Comment> = Relation::new(Self::id, Comment::post_id);
}

impl RelationKey for PostModel {
    fn relation_key(&self, column: Column) -> Option<i64> {
        match column.name() {
            "id" => Some(self.id as i64),
            "user_id" => Some(self.user_id as i64),
            _ => None,
        }
    }
}

impl RelationLoader<CommentModel> for PostModel {
    fn load_relation(&mut self, related: Vec<CommentModel>) {
        self.comments = related;
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct CommentModel {
    pub id: i32,
    pub post_id: i32,
    pub body: String,
}

pub struct Comment;

impl Entity for Comment {
    type Model = CommentModel;

    const TABLE: &'static str = "comments";

    const COLUMNS: &'static [Column] = &[
        Column::new("id"),
        Column::new("post_id"),
        Column::new("body"),
    ];
}

impl Comment {
    #[allow(non_upper_case_globals)]
    pub const id: Field<Self, i32> = Field::new("id");

    #[allow(non_upper_case_globals)]
    pub const post_id: Field<Self, i32> = Field::new("post_id");

    #[allow(non_upper_case_globals)]
    pub const body: Field<Self, String> = Field::new("body");
}

impl RelationKey for CommentModel {
    fn relation_key(&self, column: Column) -> Option<i64> {
        match column.name() {
            "id" => Some(self.id as i64),
            "post_id" => Some(self.post_id as i64),
            _ => None,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let db = PgPool::connect("postgres://postgres:admin@localhost/rustorm").await?;

    // ------------------------------------------------------------
    // REAL EAGER LOADING TEST
    // ------------------------------------------------------------

    let users_with_posts = User::find().with(User::posts).all(&db).await?;

    println!("\n=== USERS WITH POSTS ===");

    for user in &users_with_posts {
        println!("User: {} ({})", user.name, user.id);

        for post in &user.posts {
            println!("  Post: {} ({})", post.title, post.id);
        }
    }

    // ------------------------------------------------------------
    // NORMAL USER QUERY
    // ------------------------------------------------------------

    let users = User::find()
        .where_(User::name.eq("Fakhir"))
        .all(&db)
        .await?;

    // ------------------------------------------------------------
    // COUNT
    // ------------------------------------------------------------

    let user_count = User::find().select(User::id.count()).count(&db).await?;

    // ------------------------------------------------------------
    // POSTS
    // ------------------------------------------------------------

    let posts = Post::find().where_(Post::title.eq("Rust")).all(&db).await?;

    // ------------------------------------------------------------
    // COUNT WITH WHERE
    // ------------------------------------------------------------

    let user_count_with_where = User::find()
        .where_(User::name.eq("Fakhir"))
        .select(User::id.count())
        .count(&db)
        .await?;

    // ------------------------------------------------------------
    // GROUPED COUNTS
    // ------------------------------------------------------------

    let grouped_counts = User::find()
        .select(User::name.select())
        .select(User::id.count())
        .group_by(User::name.column())
        .having(User::id.count().gt(1))
        .fetch_all::<UserCount>(&db)
        .await?;

    // ------------------------------------------------------------
    // SUM
    // ------------------------------------------------------------

    let user_id_sum = User::find().select(User::id.sum()).count(&db).await?;

    let user_id_sum_with_where = User::find()
        .where_(User::name.eq("Fakhir"))
        .select(User::id.sum())
        .count(&db)
        .await?;

    // ------------------------------------------------------------
    // GROUPED SUM
    // ------------------------------------------------------------

    let grouped_sums = User::find()
        .select(User::name.select())
        .select(User::id.sum())
        .group_by(User::name.column())
        .having(User::id.sum().gt(2))
        .fetch_all::<UserSum>(&db)
        .await?;

    // ------------------------------------------------------------
    // AVG
    // ------------------------------------------------------------

    let user_avg = User::find()
        .select(User::id.avg())
        .fetch_all::<UserAvg>(&db)
        .await?;

    let posts_with_comments = Post::find().with(Post::comments).all(&db).await?;

    for post in &posts_with_comments {
        println!("Post: {} ({})", post.title, post.id);

        for comment in &post.comments {
            println!("  Comment: {}", comment.body);
        }
    }
    // ------------------------------------------------------------
    // OUTPUT
    // ------------------------------------------------------------

    println!("\n=== NORMAL USERS ===");
    println!("{users:#?}");

    println!("\nUser count: {user_count}");

    println!("\n=== POSTS ===");
    println!("{posts:#?}");

    println!("\nUser count with where: {user_count_with_where}");

    println!("\nGrouped counts: {grouped_counts:#?}");

    println!("\nUser ID sum: {user_id_sum}");

    println!("\nUser ID sum with where: {user_id_sum_with_where}");

    println!("\nGrouped sums: {grouped_sums:#?}");

    println!("\nUser ID average: {user_avg:#?}");

    Ok(())
}
