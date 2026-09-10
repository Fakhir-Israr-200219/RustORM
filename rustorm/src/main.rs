use std::sync::Arc;

use rustorm::{
    entity::{Column, Entity, RelationAccess, RelationKey, RelationLoader, SingleRelationLoader},
    field::Field,
    query::relation::{ManyToMany, ManyToOne, OneToOne, Relation}, // <-- Added ManyToMany
};

use sqlx::PgPool;
use sqlx::types::BigDecimal;

// ============================================================
// PROFILE
// ============================================================

impl RelationAccess<PostModel> for UserModel {
    fn related(&self) -> &[PostModel] {
        &self.posts
    }
    fn related_mut(&mut self) -> &mut [PostModel] {
        &mut self.posts
    }
}

impl RelationAccess<CommentModel> for PostModel {
    fn related(&self) -> &[CommentModel] {
        &self.comments
    }
    fn related_mut(&mut self) -> &mut [CommentModel] {
        &mut self.comments
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct ProfileModel {
    pub id: i32,
    pub user_id: i32,
    pub bio: String,
}

pub struct Profile;

impl Entity for Profile {
    type Model = ProfileModel;

    const TABLE: &'static str = "profiles";

    const COLUMNS: &'static [Column] = &[
        Column::new("id"),
        Column::new("user_id"),
        Column::new("bio"),
    ];
}

impl Profile {
    #[allow(non_upper_case_globals)]
    pub const id: Field<Self, i32> = Field::new("id");

    #[allow(non_upper_case_globals)]
    pub const user_id: Field<Self, i32> = Field::new("user_id");

    #[allow(non_upper_case_globals)]
    pub const bio: Field<Self, String> = Field::new("bio");
}

impl RelationKey for ProfileModel {
    fn relation_key(&self, column: Column) -> Option<i64> {
        match column.name() {
            "id" => Some(self.id as i64),
            "user_id" => Some(self.user_id as i64),
            _ => None,
        }
    }
}

// ============================================================
// ROLE (NEW - Many-to-Many)
// ============================================================

#[derive(Debug, sqlx::FromRow)]
pub struct RoleModel {
    pub id: i32,
    pub name: String,
}

pub struct Role;

impl Entity for Role {
    type Model = RoleModel;

    const TABLE: &'static str = "roles";

    const COLUMNS: &'static [Column] = &[Column::new("id"), Column::new("name")];
}

impl Role {
    #[allow(non_upper_case_globals)]
    pub const id: Field<Self, i32> = Field::new("id");

    #[allow(non_upper_case_globals)]
    pub const name: Field<Self, String> = Field::new("name");
}

impl RelationKey for RoleModel {
    fn relation_key(&self, column: Column) -> Option<i64> {
        match column.name() {
            "id" => Some(self.id as i64),
            _ => None,
        }
    }
}

// ============================================================
// USER
// ============================================================

#[derive(Debug, sqlx::FromRow)]
pub struct UserModel {
    pub id: i32,
    pub name: String,

    // ONE-TO-MANY
    #[sqlx(skip)]
    pub posts: Vec<PostModel>,

    // ONE-TO-ONE
    #[sqlx(skip)]
    pub profile: Option<Arc<ProfileModel>>,

    // MANY-TO-MANY
    #[sqlx(skip)]
    pub roles: Vec<Arc<RoleModel>>,
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

    // ONE-TO-MANY
    #[allow(non_upper_case_globals)]
    pub const posts: Relation<Self, Post> = Relation::new(Self::id, Post::user_id);

    // ONE-TO-ONE
    #[allow(non_upper_case_globals)]
    pub const profile: Relation<Self, Profile, OneToOne> =
        Relation::new(Self::id, Profile::user_id);

    // MANY-TO-MANY
    #[allow(non_upper_case_globals)]
    pub const roles: Relation<Self, Role, ManyToMany> =
        Relation::<Self, Role, ManyToMany>::many_to_many(
            Self::id,
            Role::id,
            "user_roles",
            Column::new("user_id"),
            Column::new("role_id"),
        );
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

// ONE-TO-ONE loader
impl SingleRelationLoader<Arc<ProfileModel>> for UserModel {
    fn load_relation(&mut self, related: Option<Arc<ProfileModel>>) {
        self.profile = related;
    }
}

// MANY-TO-MANY loader
impl RelationLoader<Arc<RoleModel>> for UserModel {
    fn load_relation(&mut self, related: Vec<Arc<RoleModel>>) {
        self.roles = related;
    }
}

// ============================================================
// POST
// ============================================================

#[derive(Debug, sqlx::FromRow)]
pub struct PostModel {
    pub id: i32,
    pub user_id: i32,
    pub title: String,

    // ONE-TO-MANY
    #[sqlx(skip)]
    pub comments: Vec<CommentModel>,

    // MANY-TO-ONE
    #[sqlx(skip)]
    pub user: Option<Arc<UserModel>>,
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

    // ONE-TO-MANY
    #[allow(non_upper_case_globals)]
    pub const comments: Relation<Self, Comment> = Relation::new(Self::id, Comment::post_id);

    // MANY-TO-ONE
    #[allow(non_upper_case_globals)]
    pub const user: Relation<Self, User, ManyToOne> = Relation::new(Self::user_id, User::id);
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

// MANY-TO-ONE loader
impl SingleRelationLoader<Arc<UserModel>> for PostModel {
    fn load_relation(&mut self, related: Option<Arc<UserModel>>) {
        self.user = related;
    }
}

// ============================================================
// COMMENT
// ============================================================

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

// ============================================================
// MAIN
// ============================================================

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let db = PgPool::connect("postgres://postgres:admin@localhost/rustorm").await?;

    // ------------------------------------------------------------
    // ONE-TO-MANY
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

    // ------------------------------------------------------------
    // POSTS WITH COMMENTS
    // ------------------------------------------------------------

    let posts_with_comments = Post::find().with(Post::comments).all(&db).await?;

    println!("\n=== POSTS WITH COMMENTS ===");

    for post in &posts_with_comments {
        println!("Post: {} ({})", post.title, post.id);

        for comment in &post.comments {
            println!("  Comment: {}", comment.body);
        }
    }

    // ------------------------------------------------------------
    // MANY-TO-ONE
    // ------------------------------------------------------------

    let posts_with_users = Post::find().with(Post::user).all(&db).await?;

    println!("\n=== POSTS WITH USERS ===");

    for post in &posts_with_users {
        println!("Post: {} ({})", post.title, post.id);

        if let Some(user) = &post.user {
            println!("  User: {} ({})", user.name, user.id);
        } else {
            println!("  User: None");
        }
    }

    // ------------------------------------------------------------
    // ONE-TO-ONE
    // ------------------------------------------------------------

    let users_with_profiles = User::find().with(User::profile).all(&db).await?;

    println!("\n=== USERS WITH PROFILES ===");

    for user in &users_with_profiles {
        println!("User: {} ({})", user.name, user.id);

        if let Some(profile) = &user.profile {
            println!("  Profile: {}", profile.bio);
        } else {
            println!("  Profile: None");
        }
    }

    // ------------------------------------------------------------
    // MANY-TO-MANY (NEW)
    // ------------------------------------------------------------

    let users_with_roles = User::find().with(User::roles).all(&db).await?;

    println!("\n=== USERS WITH ROLES ===");

    for user in &users_with_roles {
        println!("User: {} ({})", user.name, user.id);

        for role in &user.roles {
            println!("  Role: {} ({})", role.name, role.id);
        }
    }
    let users_with_nested_with = User::find()
        .with(User::posts.with(Post::comments))
        .all(&db)
        .await?;

    println!("Users: {}", users_with_nested_with.len());
    for user in &users_with_nested_with {
        println!(" =================>User: {} (posts: {})", user.name, user.posts.len());
        for post in &user.posts {
            println!(" =================> Post: {} (comments: {})", post.title, post.comments.len());
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
