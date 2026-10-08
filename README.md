# RustORM

**A schema-driven, Prisma-inspired ORM for Rust.**

RustORM aims to bring the developer experience of Prisma/Drizzle to the Rust ecosystem: a schema file, a CLI, and type-safe generated entities, without requiring users to write ORM-specific derive macros.

> ⚠️ **Status:** Early development. The runtime query builder, relations, CRUD operations, bulk operations, transactions, PostgreSQL execution, and the core BindValue layer are functional and tested. The separate CLI already has a working schema language, parser, validator, entity/relation generator, and create-input generation. CRUD generation and migrations are still under development.

---

# Why RustORM?

Rust has powerful database libraries and ORMs, but many approaches involve significant boilerplate, macros, or manually maintained models.

RustORM takes a schema-driven approach:

> **Define your schema once → generate your entities → write type-safe queries and CRUD operations.**

| ORM | Developer Experience | Schema-Driven | Relations |
| --- | --- | --- | --- |
| **Prisma** (TS) | ⭐⭐⭐ Excellent | ✅ Yes | ✅ Automatic |
| **Drizzle** (TS) | ⭐⭐⭐ Excellent | ✅ Yes | ✅ Easy |
| **TypeORM** (TS) | ⭐⭐ Good | ⚠️ Partial | ⚠️ Manual |
| **SeaORM** (Rust) | ⭐⭐ Good | ⚠️ Generated | ⚠️ Generated |
| **Diesel** (Rust) | ⭐ Complex | ❌ No | ⚠️ Complex |
| **SQLx** (Rust) | ⭐ Raw SQL | ❌ No | ❌ Manual |
| **RustORM** (Rust) | 🚧 In Progress | 🚧 In Progress | 🚧 In Progress |

The long-term goal is simple:

> **If you're building an Axum API, you should be able to define your database schema once and work with generated, type-safe Rust instead of maintaining repetitive ORM boilerplate manually.**

---

# Vision

## 1. Define Your Schema

The planned schema language looks roughly like:

```rust
// .rustorm/schema.rustorm

model User {
    id    Int    @id @auto
    name  String
    email String @unique
    posts Post[]
}

model Post {
    id      Int    @id @auto
    title   String
    user_id Int
    user    User @relation(fields: [user_id], references: [id])
}
```

## 2. Generate Entities and Migrations

The intended workflow is:

```bash
rustorm dev
```

## 3. Use Generated Entities

The generated API is intended to look like:

```rust
let users = User::find()
    .where_(User::name.eq("Fakhir"))
    .order_by(User::id.desc())
    .take(20)
    .all(&db)
    .await?;
```

The long-term workflow:

```text
.rustorm/schema.rustorm
          ↓
      rustorm CLI
          ↓
   Generated Entities
          ↓
 Type-safe Queries + CRUD
          ↓
      Database
```

---

# Current State

RustORM currently consists of two intentionally separate projects:

```text
Runtime
G:\Rust-ORM\rustorm

CLI
G:\RustORM-CLI
```

The runtime provides the database/query layer.

The CLI provides schema parsing and code generation.

The CLI generates code for the existing RustORM runtime; it does not reimplement the runtime.

---

# Runtime

The RustORM runtime currently provides:

- Type-safe query construction
- Parameterized SQL
- Conditions and logical expressions
- Ordering
- Pagination
- DISTINCT
- GROUP BY / HAVING
- Aggregate expressions
- SQL joins
- Subqueries
- Qualified columns
- Relation metadata
- Eager relation loading
- Single-row CRUD
- Bulk CRUD
- PostgreSQL execution
- Connection pools
- Transactions
- Multiple BindValue types
- Nullable model support

The runtime is designed to remain independent from the schema parser and code-generation layer.

---

# Entity System

The runtime currently provides:

- ✅ Generic `Entity` trait
- ✅ Associated `Model` type
- ✅ Type-safe `Field<E, T>`
- ✅ Entity-specific field definitions
- ✅ Generic query construction
- ✅ Qualified columns

Example:

```rust
pub struct User;

pub struct UserModel {
    pub id: i32,
    pub name: Option<String>,
}

impl Entity for User {
    type Model = UserModel;

    const TABLE: &'static str = "users";

    const COLUMNS: &'static [Column] = &[
        Column::new("id"),
        Column::new("name"),
    ];
}
```

---

# Conditions

Supported operators include:

- ✅ `eq`
- ✅ `not_eq`
- ✅ `gt`
- ✅ `gte`
- ✅ `lt`
- ✅ `lte`
- ✅ `and`
- ✅ `or`
- ✅ `is_null`
- ✅ `is_not_null`

Example:

```rust
let users = User::find()
    .where_(
        User::id
            .gt(10)
            .and(User::name.eq("Fakhir"))
    )
    .all(&db)
    .await?;
```

NULL conditions use SQL's dedicated NULL semantics:

```rust
User::name.is_null()
User::name.is_not_null()
```

rather than treating NULL as an ordinary equality value.

---

# Query Features

RustORM currently supports:

- ✅ `WHERE`
- ✅ `ORDER BY`
- ✅ `LIMIT`
- ✅ `OFFSET`
- ✅ `DISTINCT`
- ✅ `GROUP BY`
- ✅ `HAVING`
- ✅ `IN`
- ✅ Aggregate expressions
- ✅ Subqueries
- ✅ Subquery bind propagation
- ✅ Qualified columns
- ✅ Multiple joins
- ✅ Parameterized SQL generation
- ✅ Correct bind-value ordering

---

# Aggregates

Supported aggregate expressions include:

- ✅ `COUNT`
- ✅ `SUM`
- ✅ `AVG`
- ✅ `MIN`
- ✅ `MAX`

Example:

```rust
let stats = User::find()
    .select(User::name.select())
    .select(User::id.count())
    .group_by(User::name.column())
    .having(User::id.count().gt(2))
    .all(&db)
    .await?;
```

Aggregate SQL generation and behavior are covered by tests.

---

# Joins

RustORM currently supports:

- ✅ `INNER JOIN`
- ✅ `LEFT JOIN`
- ✅ `RIGHT JOIN`
- ✅ `FULL JOIN`
- ✅ `CROSS JOIN`
- ✅ Multiple joins
- ✅ Multiple joins of the same relation
- ✅ Relation-based joins

Example:

```rust
let rows = User::find()
    .inner_join(User::posts)
    .where_(User::name.eq("Fakhir"))
    .all(&db)
    .await?;
```

---

# Subqueries

Subqueries are supported by the query builder.

Example:

```rust
let active_ids = User::find()
    .select(User::id.select())
    .where_(User::name.eq("Fakhir"));

let users = User::find()
    .where_(User::id.in_subquery(active_ids))
    .all(&db)
    .await?;
```

The compiler handles bind propagation and placeholder offsets for subqueries.

---

# Relations

Relation metadata is implemented for:

- ✅ One-to-Many
- ✅ Many-to-One
- ✅ One-to-One
- ✅ Many-to-Many

Conceptually:

```text
User
 ├── Posts       one-to-many
 ├── Profile     one-to-one
 └── Roles       many-to-many

Post
 └── User        many-to-one
```

The relation system supports relation-specific query behavior and foreign-key filtering.

## Eager Loading

The runtime currently supports:

- ✅ `.with(...)`
- ✅ Foreign-key filtering
- ✅ Parent-key collection
- ✅ Relation loading through the query executor
- ⬜ Nested `.with(...)`

Example:

```rust
let users = User::find()
    .with(User::posts)
    .all(&db)
    .await?;
```

---

# CRUD

Core CRUD execution is functional against PostgreSQL.

## Create One

```rust
let user = User::create(
    &db,
    UserCreate {
        name: "Alice".into(),
    },
)
.await?;
```

- ✅ Single-row INSERT
- ✅ Parameterized values
- ✅ `RETURNING`
- ✅ Returns generated model
- ✅ NULL values

---

## Create Many

```rust
let users = User::create_many(
    &db,
    vec![
        UserCreate {
            name: "Alice".into(),
        },
        UserCreate {
            name: "Bob".into(),
        },
    ],
)
.await?;
```

- ✅ Bulk INSERT
- ✅ Parameterized values
- ✅ `RETURNING`
- ✅ Returns created models
- ✅ Empty input handling

NULL support in bulk CRUD is still being hardened.

---

## Update One

```rust
let user = User::update(
    &db,
    user.id,
    UserUpdate {
        name: Some("Updated".into()),
    },
)
.await?;
```

RustORM supports partial updates through optional update fields:

```rust
UserUpdate {
    name: Some("Updated".into()),
}
```

Fields set to `None` are excluded from the generated `SET` clause.

Explicit NULL updates are represented separately from omitted fields.

- ✅ Single-row UPDATE
- ✅ Partial updates
- ✅ `RETURNING`
- ✅ PostgreSQL execution
- ✅ Explicit NULL updates

---

## Update Many

```rust
let affected = User::update_many(
    &db,
    User::name.eq("Alice"),
    UserUpdate {
        name: Some("Updated".into()),
    },
)
.await?;
```

- ✅ Conditional bulk UPDATE
- ✅ Parameterized conditions
- ✅ Correct bind ordering
- ✅ Returns affected-row count

Bulk NULL handling is still being hardened.

---

## Delete One

```rust
let affected = User::delete(
    &db,
    user.id,
)
.await?;
```

- ✅ Single-row DELETE
- ✅ Parameterized ID
- ✅ Returns affected-row count

---

## Delete Many

```rust
let affected = User::delete_many(
    &db,
    User::name.eq("Alice"),
)
.await?;
```

- ✅ Conditional bulk DELETE
- ✅ Parameterized conditions
- ✅ Correct bind ordering
- ✅ Returns affected-row count

---

# Bind Values

The runtime value layer currently supports:

```rust
BindValue::String(String)
BindValue::I64(i64)
BindValue::Boolean(bool)
BindValue::F64(f64)
BindValue::DateTime(chrono::NaiveDateTime)
BindValue::Decimal(rust_decimal::Decimal)
BindValue::Json(serde_json::Value)
BindValue::Null
```

The PostgreSQL executor supports the corresponding non-NULL values through SQLx.

NULL uses explicit SQL semantics:

```text
BindValue::Null
      ↓
SQL literal NULL
      ↓
PostgreSQL NULL
      ↓
Option<T>::None
```

`BindValue::Null` is intentionally not collected as a SQL placeholder because NULL is represented directly in SQL.

The runtime currently verifies NULL behavior for single-row INSERT and UPDATE.

Additional bulk NULL and mixed placeholder-order tests remain part of the current hardening work.

---

# Transactions

RustORM's database APIs support both connection pools and SQLx PostgreSQL transactions through `Acquire<Postgres>`.

Example:

```rust
let mut tx = db.begin().await?;

let user = User::create(
    &mut tx,
    UserCreate {
        name: "Transaction User".into(),
    },
)
.await?;

User::update(
    &mut tx,
    user.id,
    UserUpdate {
        name: Some("Updated".into()),
    },
)
.await?;

User::delete(
    &mut tx,
    user.id,
)
.await?;

tx.commit().await?;
```

Transaction support covers:

- ✅ Queries
- ✅ Create
- ✅ Update
- ✅ Delete
- ✅ Create many
- ✅ Update many
- ✅ Delete many
- ✅ Relation loading

---

# SQL Safety

RustORM uses parameterized SQL for application values.

For example:

```sql
WHERE name = $1
```

rather than:

```sql
WHERE name = 'some-user-input'
```

Values are collected separately from SQL generation and bound through SQLx.

This prevents application values from being directly interpolated into generated SQL.

---

# Testing

The runtime currently has an extensive test suite covering:

- Comparison operators
- Logical `AND`
- Logical `OR`
- Ordering
- `LIMIT`
- `OFFSET`
- `DISTINCT`
- `GROUP BY`
- `HAVING`
- Aggregate expressions
- Joins
- Multiple joins
- Relation joins
- Subqueries
- Complex subqueries
- Subquery bind propagation
- Qualified columns
- Relation metadata
- Foreign-key filtering
- Bind-value ordering
- NULL expressions
- NULL INSERT
- NULL UPDATE
- Query compilation
- CRUD execution
- Bulk CRUD operations
- PostgreSQL CRUD execution
- Transactional CRUD execution
- Transactional relation loading

Current runtime verification:

```text
cargo check
    ✅

cargo test
    ✅ 109 passed
    ❌ 0 failed

cargo clippy --all-targets --all-features -- -D warnings
    ✅
```

---

# Architecture

RustORM intentionally separates the runtime and CLI.

```text
                    ┌─────────────────────┐
                    │ .rustorm/schema     │
                    └──────────┬──────────┘
                               │
                               ▼
                    ┌─────────────────────┐
                    │   rustorm CLI       │
                    │ Parser / Validator  │
                    │ Code Generator      │
                    └──────────┬──────────┘
                               │
                               ▼
                    ┌─────────────────────┐
                    │ Generated Rust      │
                    │ Models / Entities   │
                    │ Relations / CRUD    │
                    └──────────┬──────────┘
                               │
                               ▼
                    ┌─────────────────────┐
                    │ RustORM Runtime     │
                    │ Query / SQL / CRUD  │
                    └──────────┬──────────┘
                               │
                               ▼
                         PostgreSQL
```

| Layer | Project | Responsibility |
| --- | --- | --- |
| **Runtime** | `G:\Rust-ORM\rustorm` | Query engine, fields, expressions, SQL compilation, BindValue, CRUD and execution |
| **CLI** | `G:\RustORM-CLI` | Schema parsing, validation and code generation |
| **Generated code** | Produced by CLI | Models, entities, fields, relations and CRUD input types |

The CLI generates code for the existing RustORM runtime.

The runtime does not depend on the CLI.

---

# Runtime

The runtime provides:

- `Entity`
- `Query`
- `Field`
- Conditions
- SQL expressions
- SQL compilation
- Bind values
- Relation infrastructure
- CRUD execution
- Transaction-compatible execution

The runtime does not need to know an application's complete schema.

---

# CLI

The CLI is being developed separately in:

```text
G:\RustORM-CLI
```

Current CLI V0.1 provides:

- ✅ Schema language
- ✅ Lexer
- ✅ Parser
- ✅ AST / Representation
- ✅ Validator
- ✅ Entity generation
- ✅ Model generation
- ✅ Field generation
- ✅ Scalar types
- ✅ Nullable types
- ✅ Array types
- ✅ `@map`
- ✅ `@@map`
- ✅ One-to-Many relations
- ✅ Many-to-One relations
- ✅ One-to-One relations
- ✅ Many-to-Many relations
- ✅ RelationKey
- ✅ Relation-key `@map`
- ✅ Create structs

Current CLI verification:

```text
Tests:
42 passed

Clippy:
clean
```

---

# CLI CRUD Generation

The next CLI milestone is CRUD code generation.

Planned/generated pieces:

```text
Create inputs
      ↓
InsertData
      ↓
Update structs
      ↓
UpdateData
```

The runtime BindValue contract has now been expanded sufficiently to begin this work.

Nullable updates must preserve the distinction between:

```text
field omitted
```

and:

```text
field explicitly set to NULL
```

This distinction is important for generated `UpdateData` implementations.

---

# Schema

The schema language and parser are being developed in the separate CLI project.

The intended source of truth is:

```text
.rustorm/schema.rustorm
```

Example:

```rust
model User {
    id    Int    @id @auto
    name  String
    email String @unique
    posts Post[]
}
```

The schema system currently has working parsing, representation, and validation infrastructure in CLI V0.1.

---

# Generated Entities

The CLI generator currently supports:

- Model structs
- Entity definitions
- Field constants
- Scalar Rust types
- Nullable Rust types
- Array Rust types
- `@map`
- `@@map`
- One-to-Many relations
- Many-to-One relations
- One-to-One relations
- Many-to-Many relations
- RelationKey
- Relation-key `@map`
- Create structs

Generated code is intended to remain readable and inspectable Rust.

---

# Migrations

Migration support is planned but not yet implemented.

Planned functionality:

- Schema diffing
- Migration generation
- Migration execution
- Migration tracking
- Rollback support

---

# Design Principles

## 1. Schema Is the Source of Truth

The long-term goal is to describe the database schema once and derive the required Rust entities and migrations from it.

```text
schema
   ↓
generated Rust
   ↓
queries + CRUD
   ↓
database
```

---

## 2. Type-Safe Without ORM Macros

RustORM aims to provide compile-time safety without requiring users to write ORM-specific derive macros.

For example:

```rust
User::name.eq("Fakhir")
```

uses a typed Rust field rather than an arbitrary string.

---

## 3. Parameterized SQL

Application values are passed through SQL parameters:

```sql
WHERE name = $1
```

rather than being directly interpolated into SQL.

---

## 4. No Hidden Magic

Generated code should be readable Rust.

Users should be able to inspect generated files and understand what RustORM produced.

---

## 5. Works Without a Live Database

The schema-driven workflow should be able to generate entities from:

```text
.rustorm/schema.rustorm
```

without requiring a live database simply to generate Rust code.

---

## 6. Escape Hatches Matter

RustORM is not intended to eliminate SQL.

Raw SQL should remain available for queries that are:

- Too specialized
- Database-specific
- Difficult to express through the ORM
- Performance-sensitive

The goal is to make common operations easier while keeping SQL accessible.

---

## 7. Incremental Development

RustORM is being developed incrementally:

```text
Query Engine
     ↓
Relations
     ↓
CRUD
     ↓
Transactions
     ↓
BindValue
     ↓
Schema / CLI
     ↓
Code Generation
     ↓
Migrations
     ↓
CLI Workflow
```

---

# Roadmap

## Query Engine

```text
WHERE                  ✅
Operators              ✅
AND / OR               ✅
ORDER BY               ✅
LIMIT / OFFSET         ✅
DISTINCT               ✅
GROUP BY               ✅
HAVING                 ✅
Aggregates             ✅
JOINs                  ✅
CROSS JOIN             ✅
Multiple JOINs         ✅
IN                     ✅
Basic Subquery         ✅
Subquery binds         ✅
Qualified columns      ✅
COUNT execution        ⬜
```

## Relations

```text
One-to-Many             ✅
Many-to-One             ✅
One-to-One              ✅
Many-to-Many            ✅
.with()                 ✅
Eager loading           ✅
Nested .with()          ⬜
```

## CRUD

```text
INSERT                  ✅
UPDATE                  ✅
DELETE                  ✅
INSERT MANY             ✅
UPDATE MANY             ✅
DELETE MANY             ✅
```

## Bind Values

```text
String                  ✅
I64                     ✅
Boolean                 ✅
F64                     ✅
DateTime                ✅
Decimal                 ✅
JSON                    ✅
NULL                    ✅
PostgreSQL binding      ✅
Single-row NULL tests   ✅
Bulk NULL tests         ⬜
Mixed bind ordering     ⬜
```

## Runtime

```text
Executor                ✅
PostgreSQL              ✅
Pool                    ✅
Transactions            ✅
ORM-level errors        ⬜
```

## Advanced SQL

```text
CTE                     ⬜
UNION                   ⬜
FOR UPDATE              ⬜
```

## Schema / CLI

```text
Schema language         ✅
Lexer                   ✅
Parser                  ✅
Representation          ✅
Validation              ✅
```

## Generator

```text
Entities                ✅
Models                  ✅
Fields                  ✅
Relations               ✅
Create inputs           ✅
InsertData              ⬜
Update inputs           ⬜
UpdateData              ⬜
Client API              ⬜
```

## Migrations

```text
Generate                ⬜
Run                     ⬜
Rollback                ⬜
Migration tracking      ⬜
Schema diffing          ⬜
```

## CLI

```text
init                    ⬜
dev                     ⬜
generate                ⬜
migration               ⬜
```

## Quality

```text
Test extraction         ⬜
Visibility cleanup      🚧
Clippy                  ✅
Documentation           ⬜
Multi-DB architecture   ⬜
```

---

# Current Project Structure

## Runtime

```text
G:\Rust-ORM\rustorm

src/
├── entity/
│   └── mod.rs
├── executor/
│   └── mod.rs
├── field/
│   └── mod.rs
├── query/
│   ├── builder.rs
│   ├── condition.rs
│   ├── expression.rs
│   ├── join.rs
│   ├── mod.rs
│   ├── relation.rs
│   └── statement.rs
├── sql/
│   ├── mod.rs
│   └── postgres.rs
├── tests/
│   ├── mod.rs
│   └── models.rs
├── value/
│   └── mod.rs
├── lib.rs
└── main.rs
```

Runtime responsibilities:

```text
Entity
   │
   ├── Field
   │
   ├── Query
   │      ├── Conditions
   │      ├── Expressions
   │      ├── Joins
   │      └── Relations
   │
   ├── SQL Compiler
   │
   ├── Bind Values
   │
   └── Executor
          ├── Create
          ├── Update
          ├── Delete
          └── Transactions
```

## CLI

```text
G:\RustORM-CLI
```

The CLI is maintained independently from the runtime.

---

# Getting Started

> ⚠️ The schema-driven CLI workflow is still under development. The current runtime can be used with manually defined entities.

## PostgreSQL

RustORM currently targets PostgreSQL through SQLx.

Example dependencies:

```toml
[dependencies]
rustorm = "0.1"

sqlx = { version = "0.8", features = [
    "runtime-tokio",
    "postgres",
    "macros",
    "rust_decimal",
    "chrono",
    "json"
] }

tokio = { version = "1", features = [
    "macros",
    "rt-multi-thread"
] }

chrono = "0.4"
rust_decimal = "1"
serde_json = "1"
```

## Define an Entity

The current runtime API uses the `Entity` trait:

```rust
use rustorm::{
    entity::{Column, Entity},
    field::Field,
};

pub struct User;

pub struct UserModel {
    pub id: i32,
    pub name: Option<String>,
}

impl Entity for User {
    type Model = UserModel;

    const TABLE: &'static str = "users";

    const COLUMNS: &'static [Column] = &[
        Column::new("id"),
        Column::new("name"),
    ];
}

impl User {
    pub const id: Field<Self, i32> =
        Field::new("id");

    pub const name: Field<Self, String> =
        Field::new("name");
}
```

## Query the Entity

```rust
let users = User::find()
    .where_(User::name.eq("Fakhir"))
    .take(20)
    .all(&db)
    .await?;
```

---

# Example

A current RustORM application can use:

```rust
let user = User::create(
    &db,
    UserCreate {
        name: "Fakhir".into(),
    },
)
.await?;

let users = User::find()
    .where_(User::name.eq("Fakhir"))
    .order_by(User::id.desc())
    .take(20)
    .all(&db)
    .await?;

User::update_many(
    &db,
    User::name.eq("Old Name"),
    UserUpdate {
        name: Some("New Name".into()),
    },
)
.await?;

User::delete(
    &db,
    user.id,
)
.await?;
```

The intended final developer experience is:

```text
Schema
  ↓
Generated Rust
  ↓
Type-safe Queries + CRUD
  ↓
Parameterized SQL
  ↓
Database
```

---

# What's Next?

The immediate development focus is the **CLI CRUD generation layer**.

```text
G:\RustORM-CLI

Existing
   ↓
Schema language
   ↓
Lexer
   ↓
Parser
   ↓
Representation
   ↓
Validation
   ↓
Entity / Relation generation
   ↓
Create structs
   ↓
────────────────────
NEXT
   ↓
InsertData generation
   ↓
Update structs
   ↓
UpdateData generation
```

The runtime BindValue contract has already been expanded to support the types required by the generated CRUD layer.

The next goal is to make generated CRUD code compile cleanly against the existing RustORM runtime.

---

# Contributing

RustORM is an early-stage project, and feedback is welcome.

Areas where contributions and discussion are especially useful:

### Schema Language

Help design the `.rustorm` schema syntax:

```rust
model User {
    id    Int    @id @auto
    name  String
    posts Post[]
}
```

Ideas around:

- Types
- Attributes
- Relations
- Constraints
- Defaults
- Indexes

### CLI Ergonomics

Feedback is welcome around commands such as:

```bash
rustorm init
rustorm dev
rustorm generate
rustorm migrate
```

### Framework Integrations

Examples and real-world use cases for:

- Axum
- Actix Web
- Other Rust web frameworks

---

# Project Philosophy

RustORM is not trying to hide the database from Rust developers.

The goal is to remove repetitive ORM work while keeping:

- Rust's type safety
- SQL visibility
- Generated-code transparency
- Escape hatches
- Predictable developer experience

The long-term vision is:

```text
Schema
  ↓
Generated Rust
  ↓
Type-safe Queries + CRUD
  ↓
Parameterized SQL
  ↓
Database
```

> **The goal: Prisma-like ergonomics for Rust, while keeping Rust and SQL visible underneath.**

---

# License

License: **TBD**

---

# Acknowledgements

RustORM is inspired by:

- [Prisma](https://www.prisma.io/)
- [Drizzle](https://orm.drizzle.team/)
- Entity Framework
- Rust's database and ORM ecosystem

The project is also inspired by the frustration of writing macros and repetitive boilerplate for concepts as ordinary as fields, entities, and foreign-key relationships.

---

# Status

RustORM is currently in **early development**.

The runtime query engine, relations, CRUD operations, bulk operations, transaction support, PostgreSQL execution, and BindValue layer are functional and tested.

Current runtime verification:

```text
cargo check
    ✅

cargo test
    ✅ 109 passed
    ❌ 0 failed

cargo clippy --all-targets --all-features -- -D warnings
    ✅
```

The separate CLI currently has:

```text
Schema language        ✅
Lexer                  ✅
Parser                 ✅
Representation         ✅
Validator              ✅
Entity generator       ✅
Relation generator     ✅
Create structs         ✅

Tests                  42 passed
Clippy                 clean
```

The immediate next milestone is:

```text
CLI CRUD generation
        ↓
InsertData
        ↓
Update inputs
        ↓
UpdateData
```

The long-term goal remains:

```text
.rustorm/schema.rustorm
          ↓
       rustorm CLI
          ↓
   Generated Entities
          ↓
 Type-safe Queries + CRUD
          ↓
       Database
```

> **The goal: Prisma-like ergonomics for Rust, while keeping Rust and SQL visible underneath.**