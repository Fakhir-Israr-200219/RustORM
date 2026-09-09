use crate::entity::{RelationKey, RelationLoader, SingleRelationLoader};
use crate::query::relation::{ManyToMany, ManyToOne, OneToOne, Relation};
use crate::{Column, Entity, field::Field};
use std::sync::Arc;

// ============= TestUser =============
pub struct TestUser;

#[derive(Debug)]
pub struct TestUserModel {
    pub id: i32,
    #[allow(dead_code)]
    pub name: String,
    pub posts: Vec<TestPostModel>,
    pub profile: Option<Arc<TestProfileModel>>,
    pub roles: Vec<Arc<TestRoleModel>>,
}

impl RelationLoader<TestPostModel> for TestUserModel {
    fn load_relation(&mut self, related: Vec<TestPostModel>) {
        self.posts = related;
    }
}

impl SingleRelationLoader<Arc<TestProfileModel>> for TestUserModel {
    fn load_relation(&mut self, related: Option<Arc<TestProfileModel>>) {
        self.profile = related;
    }
}

impl RelationLoader<Arc<TestRoleModel>> for TestUserModel {
    fn load_relation(&mut self, related: Vec<Arc<TestRoleModel>>) {
        self.roles = related;
    }
}

impl RelationKey for TestUserModel {
    fn relation_key(&self, column: Column) -> Option<i64> {
        match column.name() {
            "id" => Some(self.id as i64),
            _ => None,
        }
    }
}

impl Entity for TestUser {
    type Model = TestUserModel;
    const TABLE: &'static str = "users";
    const COLUMNS: &'static [Column] = &[Column::new("id"), Column::new("name")];
}

impl TestUser {
    #[allow(non_upper_case_globals)]
    pub const id: Field<Self, i32> = Field::new("id");

    #[allow(non_upper_case_globals)]
    pub const name: Field<Self, String> = Field::new("name");

    #[allow(non_upper_case_globals)]
    pub const posts: Relation<Self, TestPost> =
        Relation::new(Self::id, TestPost::user_id);

    #[allow(non_upper_case_globals)]
    pub const profile: Relation<Self, TestProfile, OneToOne> =
        Relation::new(Self::id, TestProfile::user_id);

    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    pub const roles: Relation<Self, TestRole, ManyToMany> =
        Relation::<Self, TestRole, ManyToMany>::many_to_many(
            Self::id,
            TestRole::id,
            "user_roles",
            Column::new("user_id"),
            Column::new("role_id"),
        );
}

// ============= TestPost =============
pub struct TestPost;

#[derive(Debug)]
pub struct TestPostModel {
    pub id: i32,
    #[allow(dead_code)]
    pub title: String,
    pub user_id: i32,
    pub comments: Vec<TestCommentModel>,
    pub user: Option<Arc<TestUserModel>>,
}

impl SingleRelationLoader<Arc<TestUserModel>> for TestPostModel {
    fn load_relation(&mut self, related: Option<Arc<TestUserModel>>) {
        self.user = related;
    }
}

impl RelationLoader<TestCommentModel> for TestPostModel {
    fn load_relation(&mut self, related: Vec<TestCommentModel>) {
        self.comments = related;
    }
}

impl RelationKey for TestPostModel {
    fn relation_key(&self, column: Column) -> Option<i64> {
        match column.name() {
            "id" => Some(self.id as i64),
            "user_id" => Some(self.user_id as i64),
            _ => None,
        }
    }
}

impl Entity for TestPost {
    type Model = TestPostModel;
    const TABLE: &'static str = "posts";
    const COLUMNS: &'static [Column] = &[
        Self::id.column(),
        Self::title.column(),
        Self::user_id.column(),
    ];
}

impl TestPost {
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    pub const id: Field<Self, i32> = Field::new("id");

    #[allow(non_upper_case_globals)]
    pub const user_id: Field<Self, i32> = Field::new("user_id");

    #[allow(non_upper_case_globals)]
    pub const title: Field<Self, String> = Field::new("title");

    #[allow(non_upper_case_globals)]
    pub const comments: Relation<Self, TestComment> = Relation::new(Self::id, TestComment::post_id);

    #[allow(non_upper_case_globals)]
    pub const user: Relation<Self, TestUser, ManyToOne> =
        Relation::new(Self::user_id, TestUser::id);
}

// ============= TestComment =============
pub struct TestComment;

#[derive(Debug)]
pub struct TestCommentModel {
    #[allow(dead_code)]
    pub id: i32,
    #[allow(dead_code)]
    pub body: String,
    #[allow(dead_code)]
    pub post_id: i32,
}

impl Entity for TestComment {
    type Model = TestCommentModel;
    const TABLE: &'static str = "comments";
    const COLUMNS: &'static [Column] = &[Column::new("id"), Column::new("body")];
}

impl TestComment {
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    pub const id: Field<Self, i32> = Field::new("id");

    #[allow(non_upper_case_globals)]
    pub const post_id: Field<Self, i32> = Field::new("post_id");

    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    pub const body: Field<Self, String> = Field::new("body");
}

// ============= TestRole (NEW - Many-to-Many) =============
pub struct TestRole;

#[derive(Debug)]
pub struct TestRoleModel {
    pub id: i32,
    #[allow(dead_code)]
    pub name: String,
}

impl RelationKey for TestRoleModel {
    fn relation_key(&self, column: Column) -> Option<i64> {
        match column.name() {
            "id" => Some(self.id as i64),
            _ => None,
        }
    }
}

impl Entity for TestRole {
    type Model = TestRoleModel;
    const TABLE: &'static str = "roles";
    const COLUMNS: &'static [Column] = &[Self::id.column(), Self::name.column()];
}

impl TestRole {
    #[allow(non_upper_case_globals)]
    pub const id: Field<Self, i32> = Field::new("id");

    #[allow(non_upper_case_globals)]
    pub const name: Field<Self, String> = Field::new("name");
}

// ============= TestProfile =============
pub struct TestProfile;

#[derive(Debug)]
pub struct TestProfileModel {
    pub id: i32,
    pub user_id: i32,
    #[allow(dead_code)]
    pub bio: String,
}

impl RelationKey for TestProfileModel {
    fn relation_key(&self, column: Column) -> Option<i64> {
        match column.name() {
            "id" => Some(self.id as i64),
            "user_id" => Some(self.user_id as i64),
            _ => None,
        }
    }
}

impl Entity for TestProfile {
    type Model = TestProfileModel;
    const TABLE: &'static str = "profiles";
    const COLUMNS: &'static [Column] = &[
        Column::new("id"),
        Column::new("user_id"),
        Column::new("bio"),
    ];
}

impl TestProfile {
    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    pub const id: Field<Self, i32> = Field::new("id");

    #[allow(non_upper_case_globals)]
    pub const user_id: Field<Self, i32> = Field::new("user_id");

    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    pub const bio: Field<Self, String> = Field::new("bio");
}
