# RustORM

**A schema-driven, Prisma-inspired ORM for Rust.**

RustORM aims to bring the developer experience of Prisma/Drizzle to the Rust ecosystem: a schema file, a CLI, and type-safe generated entities, without requiring users to write ORM-specific derive macros.

> ⚠️ **Status:** Active development. The runtime query builder, relations, CRUD operations, bulk operations, transactions, PostgreSQL execution, BindValue layer, and CLI CRUD generation are functional and tested. The separate CLI has a working schema language, parser, validator, entity/relation generator, migration generator, full CRUD code generation, watch mode, migration execution, and `.env`/`config.toml` support. End-to-end verification complete with Axum example.

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
| **RustORM** (Rust) | 🚧 Active Dev | ✅ Yes | ✅ Yes |

The long-term goal is simple:

> **If you're building an Axum API, you should be able to define your database schema once and work with generated, type-safe Rust instead of maintaining repetitive ORM boilerplate manually.**

---

# Vision

## 1. Define Your Schema

The schema language looks like:

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

```bash
rustorm init          # Initialize project with .rustorm/ folder
rustorm dev           # Watch mode: auto-regenerate on schema change
rustorm migrate --apply  # Apply migration to database (reads .env automatically)
```

## 3. Use Generated Entities

```rust
let users = User::find()
    .where_(User::name.eq("Fakhir"))
    .order_by(User::id.desc())
    .take(20)
    .all(&db)
    .await?;
```

The workflow:

```text
.rustorm/schema.rustorm
          ↓
      rustorm CLI
          ↓
   Generated Entities + Migrations
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
- **NULL handling in bulk operations**
- **Mixed NULL/non-NULL placeholder ordering**

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
- ✅ **NULL values in bulk operations**
- ✅ **Mixed NULL/non-NULL placeholder ordering**

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
- ✅ **NULL updates in bulk operations**
- ✅ **Mixed NULL/non-NULL placeholder ordering**

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

The runtime verifies NULL behavior for:
- ✅ Single-row INSERT and UPDATE
- ✅ Bulk INSERT with NULL values
- ✅ Bulk UPDATE with NULL values
- ✅ Mixed NULL/non-NULL placeholder ordering

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
- **Bulk NULL INSERT**
- **Bulk NULL UPDATE**
- **Mixed NULL/non-NULL placeholder ordering**
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
    ✅ 114 passed
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
                    │ Migration Generator │
                    └──────────┬──────────┘
                               │
                               ▼
                    ┌─────────────────────┐
                    │ Generated Rust      │
                    │ Models / Entities   │
                    │ Relations / CRUD    │
                    │ Migration SQL       │
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
| **CLI** | `G:\RustORM-CLI` | Schema parsing, validation, code generation, migration generation, watch mode, and migration execution |
| **Generated code** | Produced by CLI | Models, entities, fields, relations, CRUD input types, and migration SQL |

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

Current CLI provides:

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
- ✅ **InsertData generation**
- ✅ **Update structs**
- ✅ **UpdateData generation**
- ✅ **Nullable update semantics (Option<Option<T>>)**
- ✅ **Migration SQL generation**
- ✅ **Shared naming module (table/column conventions)**
- ✅ **Serde Serialize/Deserialize derives on generated types**
- ✅ **`rustorm init` command (project scaffolding)**
- ✅ **`rustorm dev` command (watch mode with auto-regeneration)**
- ✅ **`rustorm migrate --apply` (direct database execution)**
- ✅ **`.env` file support (auto-read database URL)**
- ✅ **`config.toml` support (structured configuration)**
- ✅ **`.gitignore` generation (protects .env from git)**
- ✅ **Snapshot-based change detection**

Current CLI verification:

```text
Tests:
61 passed

Clippy:
clean
```

---

# CLI CRUD Generation

**✅ COMPLETE** — CLI CRUD code generation is now functional.

Generated pieces:

```text
Create inputs
      ↓
InsertData
      ↓
Update structs
      ↓
UpdateData
```

The runtime BindValue contract has been expanded to support all required types:

- String
- I64
- Boolean
- F64
- DateTime
- Decimal
- JSON
- NULL

Nullable updates preserve the distinction between:

```text
field omitted
```

and:

```text
field explicitly set to NULL
```

This distinction is important for generated `UpdateData` implementations and uses `Option<Option<T>>` semantics.

Generated model structs include `serde::Serialize` and `serde::Deserialize` derives, enabling direct use with web frameworks like Axum without wrapper types.

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

The schema system currently has working parsing, representation, and validation infrastructure.

---

# Generated Entities

The CLI generator currently supports:

- Model structs (with `Serialize`, `Deserialize`, `FromRow`)
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
- Create structs (with `Serialize`, `Deserialize`)
- **InsertData implementations**
- **Update structs (with `Serialize`, `Deserialize`)**
- **UpdateData implementations**

Generated code is intended to remain readable and inspectable Rust.

---

# Migrations

Migration support is partially implemented.

Current functionality:

- ✅ **Migration SQL generation** (CREATE TABLE, FOREIGN KEY, indexes)
- ✅ **Migration execution** (`rustorm migrate --apply`)
- ⬜ Rollback support
- ⬜ Migration tracking table
- ⬜ Schema diffing (incremental migrations)

The CLI can generate initial migration SQL from a schema file and apply it directly to a PostgreSQL database.

---

# Naming Conventions

The CLI uses a shared naming module for consistent conventions:

- **Table names:** pluralized snake_case (e.g., `User` → `users`, `UserProfile` → `user_profiles`)
- **Column names:** snake_case (e.g., `userId` → `user_id`, `createdAt` → `created_at`)
- **`@map` overrides:** respected for both table and column names

This ensures consistency between generated entities and migration SQL.

---

# Configuration

The CLI supports multiple configuration sources for database connection strings:

**Priority order:** CLI argument > `.env` file > `config.toml`

### `.env` File (Recommended for Local Dev)

Generated automatically by `rustorm init`:

```text
# .rustorm/.env
DATABASE_URL=postgres://postgres:admin@localhost/mydb
```

Protected by `.gitignore` to prevent credential leaks.

### `config.toml` (Structured Configuration)

Generated automatically by `rustorm init`:

```toml
[database]
url = "postgres://postgres:admin@localhost/mydb"
```

### CLI Argument

```bash
rustorm migrate --apply --database postgres://...
```

---

# End-to-End Verification

**✅ VERIFIED** — Full workflow tested:

```text
rustorm init
      ↓
.rustorm/schema.rustorm (edit schema)
      ↓
rustorm dev (watch mode → auto-regenerate)
      ↓
.rustorm/generated/entities.rs
.rustorm/migrations/0001_initial.sql
      ↓
rustorm migrate --apply (reads .env automatically)
      ↓
PostgreSQL: CREATE / READ / UPDATE / DELETE
      ↓
Axum API serving JSON responses
      ↓
✅ ALL OPERATIONS PASSED
```

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
CLI CRUD Generation ✅
     ↓
Migrations ✅
     ↓
CLI Workflow (init, dev) ✅
     ↓
Migration Execution ✅
     ↓
Axum Example ✅
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
Bulk NULL tests         ✅
Mixed bind ordering     ✅
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
InsertData              ✅
Update inputs           ✅
UpdateData              ✅
Serde derives           ✅
Client API              ⬜
```

## Migrations

```text
Generate                ✅
Run (--apply)           ✅
Rollback                ⬜
Migration tracking      ⬜
Schema diffing          ⬜
```

## CLI

```text
init                    ✅
dev (watch mode)        ✅
generate                ✅
migrate                 ✅
migrate --apply         ✅
.env support            ✅
config.toml support     ✅
.gitignore generation   ✅
Snapshot detection      ✅
```

## Quality

```text
Test extraction         ⬜
Visibility cleanup      🚧
Clippy                  ✅
Documentation           🚧
Multi-DB architecture   ⬜
End-to-end verification ✅
Axum example            ✅
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

src/
├── main.rs
├── lexer.rs
├── parser.rs
├── ast.rs
├── validator.rs
├── generator.rs
├── migration.rs
└── naming.rs (shared conventions)
```

The CLI is maintained independently from the runtime.

---

# Getting Started

> ✅ The schema-driven CLI workflow is fully functional.

## Prerequisites

- Rust toolchain
- PostgreSQL database

## Quick Start

```bash
# Install the CLI
cargo install --path .  # or from crates.io when published

# Initialize a new project
mkdir my-api && cd my-api
cargo init
rustorm init

# Edit your schema
# .rustorm/schema.rustorm is ready with a template

# Start watch mode (auto-regenerates on save)
rustorm dev

# In another terminal, apply migrations
rustorm migrate --apply
```

## Project Structure After Init

```text
my-api/
├── Cargo.toml
├── src/
│   └── main.rs
└── .rustorm/
    ├── schema.rustorm      ← Edit your schema here
    ├── .env                ← Database URL (edit this)
    ├── config.toml         ← Advanced configuration
    ├── .gitignore          ← Protects .env from git
    ├── generated/
    │   └── entities.rs     ← Auto-generated (do not edit)
    └── migrations/
        ├── 0001_initial.sql
        └── snapshot.json
```

## Dependencies

Add to your `Cargo.toml`:

```toml
[dependencies]
rustorm = { path = "../Rust-ORM/rustorm" }  # or from crates.io

sqlx = { version = "0.8", features = [
    "runtime-tokio",
    "postgres",
    "macros",
    "rust_decimal",
    "chrono",
    "json"
] }

tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
chrono = "0.4"
rust_decimal = "1"
serde_json = "1"
```

## Use Generated Entities

```rust
mod entities {
    include!("../.rustorm/generated/entities.rs");
}

use entities::{User, UserCreate, UserUpdate};
use rustorm::Entity;
use sqlx::PgPool;

async fn example(pool: &PgPool) {
    // Create
    let user = User::create(
        pool,
        UserCreate { name: "Fakhir".into(), email: "fakhir@test.com".into() },
    ).await.unwrap();

    // Query
    let users = User::find()
        .where_(User::name.eq("Fakhir"))
        .order_by(User::id.desc())
        .take(20)
        .all(pool)
        .await.unwrap();

    // Update
    User::update(
        pool,
        user.id,
        UserUpdate { name: Some("Updated".into()), email: None },
    ).await.unwrap();

    // Delete
    User::delete(pool, user.id).await.unwrap();
}
```

---

# Axum Example

A complete Axum REST API using RustORM generated entities:

```rust
mod entities {
    include!("../.rustorm/generated/entities.rs");
}

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use entities::{User, UserCreate, UserUpdate};
use rustorm::Entity;
use sqlx::PgPool;

#[derive(Clone)]
struct AppState { db: PgPool }

async fn list_users(State(state): State<AppState>) -> Json<Vec<entities::UserModel>> {
    Json(User::find().all(&state.db).await.unwrap_or_default())
}

async fn create_user(
    State(state): State<AppState>,
    Json(input): Json<UserCreate>,
) -> Result<(StatusCode, Json<entities::UserModel>), StatusCode> {
    let user = User::create(&state.db, input).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok((StatusCode::CREATED, Json(user)))
}

async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(input): Json<UserUpdate>,
) -> Result<Json<entities::UserModel>, StatusCode> {
    let user = User::update(&state.db, id, input).await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    Ok(Json(user))
}

async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> StatusCode {
    match User::delete(&state.db, id).await {
        Ok(n) if n > 0 => StatusCode::NO_CONTENT,
        _ => StatusCode::NOT_FOUND,
    }
}

#[tokio::main]
async fn main() {
    let db_url = std::fs::read_to_string(".rustorm/.env")
        .ok()
        .and_then(|c| c.lines()
            .find(|l| l.starts_with("DATABASE_URL="))
            .map(|l| l.strip_prefix("DATABASE_URL=").unwrap().to_string()))
        .expect("DATABASE_URL not found");

    let pool = PgPool::connect(&db_url).await.expect("DB connection failed");

    let app = Router::new()
        .route("/users", get(list_users).post(create_user))
        .route("/users/{id}", put(update_user).delete(delete_user))
        .with_state(AppState { db: pool });

    println!("🚀 Server running at http://localhost:3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

**Key point:** Generated types (`UserModel`, `UserCreate`, `UserUpdate`) include `Serialize` and `Deserialize` derives — no wrapper types needed. They work directly with Axum's `Json` extractor.

Full working example: [`rustorm-example-axum`](https://github.com/Fakhir-Israr-200219/rustorm-example-axum)

---

# What's Next?

The immediate development focus is:

1. **Quality improvements:**
   - Documentation (mdbook guide)
   - crates.io publication
   - Visibility cleanup

2. **Feature completeness:**
   - Nested `.with()` eager loading
   - Migration tracking table
   - Schema diffing (incremental migrations)
   - Rollback support

3. **Multi-database support:**
   - MySQL
   - SQLite

---

# Contributing

RustORM is an active development project, and feedback is welcome.

Areas where contributions and discussion are especially useful:

### CLI Ergonomics

Feedback is welcome around commands such as:

```bash
rustorm init
rustorm dev
rustorm generate
rustorm migrate --apply
```

### Framework Integrations

Examples and real-world use cases for:

- Axum ✅
- Actix Web
- Other Rust web frameworks

### Advanced Features

- Migration execution and rollback
- Schema diffing
- Multi-database support
- Performance optimizations

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

License: **MIT OR Apache-2.0** (TBD)

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

RustORM is in **active development** with major milestones completed.

The runtime query engine, relations, CRUD operations, bulk operations, transaction support, PostgreSQL execution, BindValue layer (including NULL handling in bulk operations), CLI CRUD generation, watch mode, migration execution, and `.env`/`config.toml` configuration are all functional and tested.

Current runtime verification:

```text
cargo check
    ✅

cargo test
    ✅ 114 passed
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
InsertData generation  ✅
Update structs         ✅
UpdateData generation  ✅
Serde derives          ✅
Migration generator    ✅
Migration execution    ✅
Shared naming module   ✅
rustorm init           ✅
rustorm dev (watch)    ✅
.env support           ✅
config.toml support    ✅
.gitignore generation  ✅
Snapshot detection     ✅

Tests                  61 passed
Clippy                 clean
```

**End-to-end verification:** ✅ Complete — Schema → entities → migrations → PostgreSQL CRUD → Axum API all verified.

The immediate next milestone is:

```text
Documentation + crates.io publish
        ↓
Nested .with()
        ↓
Migration tracking + rollback
        ↓
Multi-DB support
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
```

---

## Changes Made (Nothing Deleted)

| Section | Change |
|---|---|
| Status banner | Added watch mode, migration execution, .env/config.toml, Axum example |
| Vision §2 | Updated commands to `init`, `dev`, `migrate --apply` |
| CLI section | Added 8 new ✅ items (init, dev, migrate --apply, .env, config.toml, .gitignore, serde derives, snapshot) |
| CLI CRUD Generation | Added note about serde derives enabling direct Axum usage |
| Generated Entities | Added `(with Serialize, Deserialize, FromRow)` annotations |
| Migrations | Changed `Run` from ⬜ to ✅ |
| **NEW: Configuration section** | Documents .env, config.toml, CLI arg priority |
| End-to-End Verification | Updated flow to include init, dev, migrate --apply, Axum |
| Roadmap > CLI | All 9 items now ✅ |
| Roadmap > Generator | Added `Serde derives ✅` |
| Roadmap > Migrations | `Run (--apply) ✅` |
| Roadmap > Quality | Added `Axum example ✅` |
| Getting Started | Complete rewrite with quick start, project structure, dependencies |
| **NEW: Axum Example section** | Full working code showing direct generated type usage |
| What's Next | Updated to reflect completed milestones |
| Status footer | Updated CLI checklist with all new items |

