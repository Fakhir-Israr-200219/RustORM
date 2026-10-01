# RustORM

**A schema-driven, Prisma-inspired ORM for Rust.**

RustORM aims to bring the developer experience of Prisma/Drizzle to the Rust ecosystem: a schema file, a CLI, and type-safe generated entities, without the macro-heavy boilerplate common in Rust ORMs.

> ⚠️ **Status:** Early development. The core query builder is functional and heavily tested. CLI, code generation, and migrations are in progress.

---

## Why RustORM?

Rust has powerful ORM and database libraries, but many approaches require significant boilerplate, macros, or manually maintained models.

RustORM takes a different approach:

**Define your schema once → generate your entities → write type-safe queries.**

| ORM                | Developer Experience | Schema-Driven | Relations   |
| ------------------ | -------------------- | ------------- | ----------- |
| **Prisma** (TS)    | ⭐⭐⭐ Excellent        | ✅ Yes         | ✅ Automatic |
| **Drizzle** (TS)   | ⭐⭐⭐ Excellent        | ✅ Yes         | ✅ Easy      |
| **TypeORM** (TS)   | ⭐⭐ Good              | ⚠️ Partial    | ⚠️ Manual   |
| **SeaORM** (Rust)  | ⭐⭐ Verbose           | ❌ No          | ⚠️ Manual   |
| **Diesel** (Rust)  | ⭐ Complex            | ❌ No          | ⚠️ Complex  |
| **SQLx** (Rust)    | ⭐ Raw SQL            | ❌ No          | ❌ Manual    |
| **RustORM** (Rust) | 🚧 In Progress       | ✅ Yes         | 🚧 Planned  |

The goal is simple:

> **If you're building an Axum API, you should be able to wire up your entire database in a few steps instead of fighting macros and repetitive boilerplate.**

---

## The Vision

### 1. Define your schema

```rust
// .rustorm/schema.rustorm

model User {
    id    Int     @id @auto
    name  String
    email String  @unique
    posts Post[]
}

model Post {
    id      Int    @id @auto
    title   String
    user_id Int
    user    User   @relation(fields: [user_id], references: [id])
}
```

### 2. Generate entities and migrations

```bash
rustorm dev
```

### 3. Use your generated entities

```rust
let users = User::find()
    .where_(User::name.eq("Fakhir"))
    .order_by(User::id.desc())
    .take(20)
    .all(&db)
    .await?;
```

No `#[derive(Entity)]`.

No relation macros.

No manually maintained field definitions.

Just generated, readable Rust.

---

# Current State

The **runtime query builder** is the first major component of RustORM and is currently functional.

It is designed to be generic over entities, allowing the future code-generation layer to target the runtime without requiring changes to the query engine.

## What Works Today

### Query Builder

* ✅ Generic `Entity` trait with associated `Model` type
* ✅ Type-safe `Field<T>`
* ✅ Comparison operators:

  * `eq`
  * `not_eq`
  * `gt`
  * `gte`
  * `lt`
  * `lte`
* ✅ Logical composition:

  * `.and()`
  * `.or()`
* ✅ `ORDER BY`
* ✅ `LIMIT`
* ✅ `OFFSET`
* ✅ `DISTINCT`
* ✅ `GROUP BY`
* ✅ `HAVING`
* ✅ Aggregates:

  * `COUNT`
  * `SUM`
  * `AVG`
  * `MIN`
  * `MAX`
* ✅ All major join types:

  * `INNER`
  * `LEFT`
  * `RIGHT`
  * `FULL`
  * `CROSS`
* ✅ Subqueries
* ✅ Parameterized SQL generation
* ✅ Correct bind-value ordering
* ✅ `COUNT(*)` queries

### Relations

Relation metadata is supported for:

* ✅ One-to-many
* ✅ Many-to-one
* ✅ One-to-one
* ✅ Many-to-many

### Eager Loading Infrastructure

The runtime also contains infrastructure for:

* ✅ `with(...)`
* ✅ Foreign-key filtering
* ✅ Parent-key collection

End-to-end eager loading is still being developed.

---

# What's In Progress

* 🚧 `rustorm` CLI

  * `rustorm init`
  * `rustorm dev`
  * `rustorm migrate`
* 🚧 `.rustorm/schema.rustorm` parser
* 🚧 Entity code generation
* 🚧 Migration generation
* 🚧 Migration tracking table
* 🚧 Axum integration examples
* 🚧 Insert execution
* 🚧 Update execution
* 🚧 Delete execution
* 🚧 Upsert support

---

# Architecture

RustORM is divided into three main layers:

```text
your-app
    │
    ├──────────────► rustorm-entities
    │                     │
    │                     ▼
    └──────────────► rustorm (runtime)
```

| Layer         | Crate              | Responsibility                                                       |
| ------------- | ------------------ | -------------------------------------------------------------------- |
| **Runtime**   | `rustorm`          | Generic query engine, entities, fields, SQL compiler and bind values |
| **Generated** | `rustorm-entities` | Generated models, fields and relation metadata                       |
| **CLI**       | `rustorm-cli`      | Schema parsing, entity generation and migration management           |

### Runtime

The runtime crate provides the generic database abstraction:

* `Entity`
* `Model`
* `Query`
* `Field`
* SQL expression building
* SQL compilation
* Bind-value collection

The runtime does **not** know about the application's schema.

### Generated Entities

The generated crate is produced from the user's schema.

It contains:

* Model structs
* Entity definitions
* Field constants
* Relation metadata

Generated code is intended to remain readable and inspectable.

### CLI

The CLI will eventually manage the developer workflow:

```bash
rustorm init
rustorm dev
rustorm migrate
```

---

# Design Principles

## 1. Schema Is the Source of Truth

The database schema should be described in one place.

```text
schema
   ↓
entities
   ↓
queries
   ↓
migrations
```

The goal is to avoid manually synchronizing models, fields and migrations across multiple files.

---

## 2. Type-Safe Without Macros

RustORM aims to provide compile-time safety without requiring users to write ORM-specific derive macros.

For example:

```rust
User::name.eq("Fakhir")
```

The field is generated from the schema and checked by the Rust compiler.

---

## 3. Parameterized SQL Only

User-provided values are passed through SQL parameters.

```sql
WHERE name = $1
```

rather than being concatenated into SQL strings.

This keeps the query builder safe from SQL injection caused by value interpolation.

---

## 4. No Hidden Magic

Generated code should be readable Rust.

Users should be able to open the generated files and understand what RustORM produced.

---

## 5. Works Without a Live Database

The development workflow should not require a live database simply to generate entities.

For example:

```bash
rustorm dev
```

should be able to generate code from the schema even when the database is temporarily unreachable.

---

## 6. Escape Hatches Matter

ORMs should not prevent developers from using SQL when necessary.

Raw SQL should remain available for queries that are:

* too specialized
* database-specific
* difficult to express through the ORM
* performance-sensitive

RustORM is intended to make common operations easier, not eliminate SQL.

---

## 7. Incremental Scope

RustORM is being developed incrementally:

```text
Read Queries
     ↓
Writes
     ↓
Relations
     ↓
Migrations
     ↓
Eager Loading
     ↓
Integrations
```

The project avoids trying to build the entire ORM in one large rewrite.

---

# Example Queries

## Basic Filtering

```rust
let users = User::find()
    .where_(User::name.eq("Fakhir"))
    .all(&db)
    .await?;
```

## Ordering and Pagination

```rust
let users = User::find()
    .where_(User::name.eq("Fakhir"))
    .order_by(User::id.desc())
    .take(20)
    .skip(40)
    .all(&db)
    .await?;
```

## Compound Conditions

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

## Aggregation

```rust
let stats = User::find()
    .select(User::name.select())
    .select(User::id.count())
    .group_by(User::name.column())
    .having(User::id.count().gt(2))
    .all(&db)
    .await?;
```

## Joins

```rust
let rows = User::find()
    .inner_join(User::posts)
    .where_(User::name.eq("Fakhir"))
    .all(&db)
    .await?;
```

## Subqueries

```rust
let active_ids = User::find()
    .select(User::id.select())
    .where_(User::name.eq("Fakhir"));

let users = User::find()
    .where_(User::id.in_subquery(active_ids))
    .all(&db)
    .await?;
```

All supported queries generate parameterized SQL with bind values collected in the correct order.

---

# Testing

The query builder is backed by an extensive test suite.

Current tests cover:

* Comparison operators
* Logical `AND` / `OR`
* Ordering
* `LIMIT`
* `OFFSET`
* `DISTINCT`
* `GROUP BY`
* `HAVING`
* Aggregate functions
* All supported join types
* Multiple joins
* Simple subqueries
* Complex subqueries
* Relation metadata
* Bind-value ordering
* Foreign-key filtering
* Eager-loading query infrastructure

Run the test suite with:

```bash
cargo test
```

---

# Getting Started

> ⚠️ The CLI is still under development. For now, entities can be wired manually to experiment with the runtime.

## Add RustORM

```toml
[dependencies]
rustorm = "0.1"
sqlx = { version = "0.7", features = ["postgres", "runtime-tokio-native-tls"] }
tokio = { version = "1", features = ["full"] }
```

## Define an Entity

```rust
use rustorm::{Entity, Field, Query};

pub struct User;

pub struct UserModel {
    pub id: i32,
    pub name: String,
}

impl Entity for User {
    const TABLE_NAME: &'static str = "users";
    type Model = UserModel;
}

impl User {
    pub const id: Field<i32> = Field::new("id");
    pub const name: Field<String> = Field::new("name");

    pub fn find() -> Query<Self> {
        Query::new()
    }
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

# Roadmap

| Phase       | Status         | Deliverable                                                                           |
| ----------- | -------------- | ------------------------------------------------------------------------------------- |
| **Phase 0** | ✅ Complete     | Core traits, `Field`, `Query`, SQL compiler and bind values                           |
| **Phase 1** | ✅ Complete     | Full read query builder, filters, joins, aggregates, subqueries and relation metadata |
| **Phase 2** | 🚧 In Progress | `rustorm` CLI, schema parser and entity code generation                               |
| **Phase 3** | 🚧 Planned     | `insert`, `update`, `delete`, `upsert`                                                |
| **Phase 4** | 🚧 Planned     | Migration generation, diffing and tracking                                            |
| **Phase 5** | 📋 Planned     | End-to-end eager loading                                                              |
| **Phase 6** | 📋 Planned     | Axum integration and documentation                                                    |
| **Phase 7** | 📋 Planned     | Transactions, connection pooling and error handling improvements                      |

---

# Contributing

RustORM is an early-stage project, and feedback is welcome.

Areas where contributions and discussion are especially useful:

### Schema Language

Help design the `.rustorm` schema syntax.

```rust
model User {
    id    Int    @id @auto
    name  String
    posts Post[]
}
```

Open an issue with ideas for:

* Types
* Attributes
* Relations
* Constraints
* Defaults
* Indexes

### CLI Ergonomics

Feedback is welcome around commands such as:

```bash
rustorm init
rustorm dev
rustorm migrate
```

### Framework Integrations

Examples and real-world use cases for:

* Axum
* Actix Web
* Other Rust web frameworks

are especially welcome.

---

# Project Philosophy

RustORM is not trying to hide the database from Rust developers.

The goal is to remove repetitive ORM work while keeping:

* Rust's type safety
* SQL visibility
* Generated-code transparency
* Escape hatches
* A predictable developer experience

The long-term vision is:

```text
Schema
  ↓
Generated Rust
  ↓
Type-safe Queries
  ↓
Parameterized SQL
  ↓
Database
```

---

# License

License: **TBD**

---

# Acknowledgements

RustORM is inspired by:

* [Prisma](https://www.prisma.io/)
* [Drizzle](https://orm.drizzle.team/)
* Entity Framework

The project is also inspired by the frustration of writing macros and repetitive boilerplate for concepts as ordinary as fields, entities and foreign-key relationships.

---

## Status

RustORM is currently in **early development**.

The query builder is functional and tested.

The next major milestone is the schema-driven developer workflow:

```text
.rustorm/schema.rustorm
          ↓
     rustorm CLI
          ↓
   Generated Entities
          ↓
    Type-safe Queries
          ↓
       Database
```

**The goal: Prisma-like ergonomics for Rust, while keeping Rust and SQL visible underneath.**
