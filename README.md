# RustORM

**A schema-driven, Prisma-inspired ORM for Rust.**

RustORM aims to bring the developer experience of Prisma/Drizzle to the Rust ecosystem: a schema file, a CLI, and type-safe generated entities, without requiring users to write ORM-specific derive macros.

> ⚠️ **Status:** Early development. The runtime query builder and core CRUD operations are functional and tested. Schema-driven CLI, code generation, migrations, and end-to-end eager loading are still in development.

---

## Why RustORM?

Rust has powerful database libraries and ORMs, but many approaches involve significant boilerplate, macros, or manually maintained models.

RustORM takes a schema-driven approach:

**Define your schema once → generate your entities → write type-safe queries and CRUD operations.**

| ORM                | Developer Experience | Schema-Driven | Relations      |
| ------------------ | -------------------- | ------------- | -------------- |
| **Prisma** (TS)    | ⭐⭐⭐ Excellent        | ✅ Yes         | ✅ Automatic    |
| **Drizzle** (TS)   | ⭐⭐⭐ Excellent        | ✅ Yes         | ✅ Easy         |
| **TypeORM** (TS)   | ⭐⭐ Good              | ⚠️ Partial    | ⚠️ Manual      |
| **SeaORM** (Rust)  | ⭐⭐ Verbose           | ❌ No          | ⚠️ Manual      |
| **Diesel** (Rust)  | ⭐ Complex            | ❌ No          | ⚠️ Complex     |
| **SQLx** (Rust)    | ⭐ Raw SQL            | ❌ No          | ❌ Manual       |
| **RustORM** (Rust) | 🚧 In Progress       | 🚧 Planned    | 🚧 In Progress |

The goal is simple:

> **If you're building an Axum API, you should be able to define your database schema once and work with generated, type-safe Rust instead of maintaining repetitive ORM boilerplate manually.**

---

# The Vision

## 1. Define Your Schema

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

## 2. Generate Entities and Migrations

```bash
rustorm dev
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

The intended workflow is:

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

---

# Current State

The **runtime query engine** is functional and heavily tested.

RustORM currently supports:

* Type-safe query construction
* Parameterized SQL
* Conditions and logical expressions
* Aggregates
* Joins
* Subqueries
* Relation metadata
* CRUD execution against PostgreSQL
* Bulk CRUD operations

The runtime is designed to remain independent from the future schema parser and code-generation layer.

---

# What Works Today

## Query Builder

### Entity System

* ✅ Generic `Entity` trait
* ✅ Associated `Model` type
* ✅ Type-safe `Field<E, T>`
* ✅ Entity-specific field definitions
* ✅ Generic query construction

### Conditions

* ✅ `eq`
* ✅ `not_eq`
* ✅ `gt`
* ✅ `gte`
* ✅ `lt`
* ✅ `lte`
* ✅ `.and()`
* ✅ `.or()`
* ✅ Parameterized condition values
* ✅ Bind-value collection

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

---

## Query Features

* ✅ `WHERE`
* ✅ `ORDER BY`
* ✅ `LIMIT`
* ✅ `OFFSET`
* ✅ `DISTINCT`
* ✅ `GROUP BY`
* ✅ `HAVING`
* ✅ Aggregate expressions
* ✅ Subqueries
* ✅ Parameterized SQL generation
* ✅ Correct bind-value ordering

---

## Aggregates

Supported aggregate expressions include:

* ✅ `COUNT`
* ✅ `SUM`
* ✅ `AVG`
* ✅ `MIN`
* ✅ `MAX`

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

---

# Joins

RustORM currently supports the major SQL join types:

* ✅ `INNER JOIN`
* ✅ `LEFT JOIN`
* ✅ `RIGHT JOIN`
* ✅ `FULL JOIN`
* ✅ `CROSS JOIN`

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

Subqueries are supported through the query builder.

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

---

# Relations

Relation metadata is currently implemented for:

* ✅ One-to-many
* ✅ Many-to-one
* ✅ One-to-one
* ✅ Many-to-many

Example conceptual relationships:

```text
User
 ├── Posts       (one-to-many)
 ├── Profile     (one-to-one)
 └── Roles       (many-to-many)

Post
 └── User        (many-to-one)
```

---

# Eager Loading Infrastructure

The runtime contains infrastructure for relation loading, including:

* ✅ `with(...)`
* ✅ Foreign-key filtering
* ✅ Parent-key collection
* 🚧 Complete end-to-end eager-loading API

The underlying relation infrastructure is being developed independently from the query builder.

---

# CRUD Operations

Core CRUD execution is now functional against PostgreSQL.

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

* ✅ Single-row INSERT
* ✅ `RETURNING`
* ✅ Returns generated model

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

* ✅ Bulk INSERT
* ✅ Parameterized values
* ✅ `RETURNING`
* ✅ Returns created models
* 🚧 Empty-input edge-case coverage

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

A field set to `None` is not included in the generated `SET` clause.

* ✅ Single-row UPDATE
* ✅ Partial update data
* ✅ `RETURNING`
* ✅ PostgreSQL verification

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

* ✅ Conditional bulk UPDATE
* ✅ Parameterized conditions
* ✅ Returns affected-row count

---

## Delete One

```rust
let affected = User::delete(
    &db,
    user.id,
)
.await?;
```

* ✅ Single-row DELETE
* ✅ Parameterized ID
* ✅ Returns affected-row count

---

## Delete Many

```rust
let affected = User::delete_many(
    &db,
    User::name.eq("Alice"),
)
.await?;
```

* ✅ Conditional bulk DELETE
* ✅ Parameterized conditions
* ✅ Returns affected-row count

---

# SQL Safety

RustORM uses parameterized SQL for user-provided values.

For example:

```sql
WHERE name = $1
```

rather than:

```sql
WHERE name = 'some-user-input'
```

Values are collected separately from SQL generation and bound through `sqlx`.

This keeps application values out of generated SQL strings and avoids SQL injection caused by direct value interpolation.

---

# Testing

RustORM currently has an extensive test suite covering the runtime query system.

Current tests cover:

* Comparison operators
* Logical `AND`
* Logical `OR`
* Ordering
* `LIMIT`
* `OFFSET`
* `DISTINCT`
* `GROUP BY`
* `HAVING`
* Aggregate expressions
* Joins
* Multiple joins
* Subqueries
* Complex subqueries
* Relation metadata
* Foreign-key filtering
* Bind-value ordering
* Query compilation
* CRUD execution
* Bulk CRUD operations
* PostgreSQL CRUD verification

Run the test suite with:

```bash
cargo test
```

The current test suite passes successfully.

Actual PostgreSQL CRUD execution is also exercised through the project's runtime test runner.

---

# Architecture

RustORM is intended to be divided into three major layers:

```text
your-app
    │
    ├──────────────► generated entities
    │                     │
    │                     ▼
    └──────────────► rustorm runtime
```

| Layer         | Crate              | Responsibility                                                             |
| ------------- | ------------------ | -------------------------------------------------------------------------- |
| **Runtime**   | `rustorm`          | Query engine, entities, fields, SQL expressions, compilation and execution |
| **Generated** | `rustorm-entities` | Generated models, fields and relation metadata                             |
| **CLI**       | `rustorm-cli`      | Schema parsing, entity generation and migration management                 |

## Runtime

The runtime provides the generic database layer:

* `Entity`
* `Query`
* `Field`
* Conditions
* SQL expressions
* SQL compilation
* Bind values
* Relation infrastructure
* CRUD execution

The runtime does not need to know an application's complete schema.

## Generated Entities

The future generated layer will contain:

* Model structs
* Entity definitions
* Field constants
* Relation metadata
* Create/update input types

Generated code is intended to remain readable and inspectable Rust.

## CLI

The future CLI will manage the schema-driven workflow:

```bash
rustorm init
rustorm dev
rustorm migrate
```

---

# Design Principles

## 1. Schema Is the Source of Truth

The database schema should eventually be described in one place.

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

## 2. Type-Safe Without ORM Macros

RustORM aims to provide compile-time safety without requiring users to write ORM-specific derive macros.

For example:

```rust
User::name.eq("Fakhir")
```

The field is represented as a typed Rust value and checked by the Rust compiler.

---

## 3. Parameterized SQL

User-provided values are passed through SQL parameters:

```sql
WHERE name = $1
```

rather than being interpolated directly into SQL.

---

## 4. No Hidden Magic

Generated code should be readable Rust.

Users should be able to inspect generated files and understand what RustORM produced.

---

## 5. Works Without a Live Database

The future schema-driven development workflow should not require a live database simply to generate entities.

For example:

```bash
rustorm dev
```

should eventually be able to generate code from:

```text
.rustorm/schema.rustorm
```

even when the database is temporarily unreachable.

---

## 6. Escape Hatches Matter

ORMs should not prevent developers from using SQL when necessary.

Raw SQL should remain available for queries that are:

* Too specialized
* Database-specific
* Difficult to express through the ORM
* Performance-sensitive

RustORM is intended to make common operations easier, not eliminate SQL.

---

## 7. Incremental Development

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

# Example

A future RustORM application should look roughly like this:

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

# What's In Progress

The next major areas of development are:

### Schema & CLI

* 🚧 `.rustorm/schema.rustorm` parser
* 🚧 `rustorm init`
* 🚧 `rustorm dev`
* 🚧 `rustorm migrate`

### Code Generation

* 🚧 Entity generation
* 🚧 Model generation
* 🚧 Field generation
* 🚧 Relation generation
* 🚧 Create/update input generation

### Migrations

* 🚧 Schema diffing
* 🚧 Migration generation
* 🚧 Migration execution
* 🚧 Migration tracking table

### Query & CRUD Hardening

* 🚧 CRUD edge-case coverage
* 🚧 Empty-operation behavior
* 🚧 Nonexistent-record behavior
* 🚧 Additional condition integration tests
* 🚧 Transaction support

### Relations

* 🚧 Complete eager-loading API
* 🚧 Relation query improvements

### Integrations

* 🚧 Axum examples
* 🚧 Documentation
* 📋 Actix Web integration
* 📋 Additional database support

---

# Roadmap

| Phase       | Status         | Deliverable                                                                |
| ----------- | -------------- | -------------------------------------------------------------------------- |
| **Phase 0** | ✅ Complete     | Core traits, fields, query infrastructure and SQL compiler                 |
| **Phase 1** | ✅ Complete     | Read queries, filters, joins, aggregates, subqueries and relation metadata |
| **Phase 2** | 🚧 In Progress | CRUD execution and runtime hardening                                       |
| **Phase 3** | 🚧 In Progress | CLI, schema parser and entity generation                                   |
| **Phase 4** | 🚧 Planned     | Migration generation, diffing and tracking                                 |
| **Phase 5** | 🚧 Planned     | End-to-end eager loading                                                   |
| **Phase 6** | 📋 Planned     | Axum integration and documentation                                         |
| **Phase 7** | 📋 Planned     | Transactions, connection pooling and broader error-handling improvements   |

---

# Getting Started

> ⚠️ The schema-driven CLI is still under development. At the moment, entities can be defined manually to experiment with the runtime.

## PostgreSQL

RustORM currently targets PostgreSQL through `sqlx`.

Example dependency setup:

```toml
[dependencies]
rustorm = "0.1"
sqlx = { version = "0.7", features = ["postgres", "runtime-tokio-native-tls"] }
tokio = { version = "1", features = ["full"] }
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
    pub name: String,
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

# Project Structure

The current runtime is organized around the following components:

```text
src/
├── entity/
├── executor/
├── field/
├── query/
├── sql/
├── tests/
└── value/
```

The main runtime responsibilities are separated into:

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
          └── Delete
```

---

# Contributing

RustORM is an early-stage project, and feedback is welcome.

Areas where contributions and discussion are especially useful:

## Schema Language

Help design the `.rustorm` schema syntax.

```rust
model User {
    id    Int    @id @auto
    name  String
    posts Post[]
}
```

Ideas around:

* Types
* Attributes
* Relations
* Constraints
* Defaults
* Indexes

are especially useful.

## CLI Ergonomics

Feedback is welcome around commands such as:

```bash
rustorm init
rustorm dev
rustorm migrate
```

## Framework Integrations

Examples and real-world use cases for:

* Axum
* Actix Web
* Other Rust web frameworks

are welcome.

---

# Project Philosophy

RustORM is not trying to hide the database from Rust developers.

The goal is to remove repetitive ORM work while keeping:

* Rust's type safety
* SQL visibility
* Generated-code transparency
* Escape hatches
* Predictable developer experience

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

* [Prisma](https://www.prisma.io/)
* [Drizzle](https://orm.drizzle.team/)
* Entity Framework
* Rust's database and ORM ecosystem

The project is also inspired by the frustration of writing macros and repetitive boilerplate for concepts as ordinary as fields, entities and foreign-key relationships.

---

## Status

RustORM is currently in **early development**.

The runtime query builder and core CRUD execution are functional and tested.

The next major milestone is the schema-driven developer workflow:

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

**The goal: Prisma-like ergonomics for Rust, while keeping Rust and SQL visible underneath.**
