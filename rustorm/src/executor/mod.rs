use crate::{
    entity::Entity, query::condition::Condition, sql::compile_expression, value::BindValue,
};

pub trait InsertData<E: Entity> {
    fn columns(&self) -> &'static [&'static str];
    fn values(&self) -> Vec<BindValue>;
}

pub async fn insert<'c, E, D, A>(db: A, data: D) -> Result<E::Model, sqlx::Error>
where
    E: Entity,
    D: InsertData<E>,
    A: sqlx::Acquire<'c, Database = sqlx::Postgres>,
    for<'r> E::Model: sqlx::FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
{
    let mut conn = db.acquire().await?;

    let columns = data.columns();
    let values = data.values();

    let mut sql = format!("INSERT INTO {} ({}) VALUES (", E::TABLE, columns.join(", "));

    for (index, _) in values.iter().enumerate() {
        if index > 0 {
            sql.push_str(", ");
        }

        sql.push_str(&format!("${}", index + 1));
    }

    sql.push_str(") RETURNING ");

    let returning = E::columns()
        .iter()
        .map(|column| column.name())
        .collect::<Vec<_>>()
        .join(", ");

    sql.push_str(&returning);

    let mut query = sqlx::query_as::<_, E::Model>(&sql);

    for value in values {
        match value {
            BindValue::String(value) => query = query.bind(value),
            BindValue::I64(value) => query = query.bind(value),
        }
    }

    query.fetch_one(&mut *conn).await
}
pub async fn insert_many<'c, E, D, A>(db: A, data: Vec<D>) -> Result<Vec<E::Model>, sqlx::Error>
where
    E: Entity,
    D: InsertData<E>,
    A: sqlx::Acquire<'c, Database = sqlx::Postgres>,
    for<'r> E::Model: sqlx::FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
{
    if data.is_empty() {
        return Ok(Vec::new());
    }
    let mut conn = db.acquire().await?;
    let columns = data[0].columns();
    let column_count = columns.len();

    let mut sql = format!("INSERT INTO {} ({}) VALUES ", E::TABLE, columns.join(", "));

    let mut values = Vec::new();

    for (row_index, item) in data.into_iter().enumerate() {
        if row_index > 0 {
            sql.push_str(", ");
        }

        sql.push('(');

        let item_values = item.values();

        for (column_index, value) in item_values.into_iter().enumerate() {
            if column_index > 0 {
                sql.push_str(", ");
            }

            let bind_index = values.len() + 1;
            sql.push_str(&format!("${}", bind_index));
            values.push(value);
        }

        sql.push(')');

        if item.columns().len() != column_count {
            return Err(sqlx::Error::Protocol(
                "All insert_many items must have the same columns".into(),
            ));
        }
    }

    sql.push_str(" RETURNING ");

    let returning = E::columns()
        .iter()
        .map(|column| column.name())
        .collect::<Vec<_>>()
        .join(", ");

    sql.push_str(&returning);

    let mut query = sqlx::query_as::<_, E::Model>(&sql);

    for value in values {
        match value {
            BindValue::String(value) => query = query.bind(value),
            BindValue::I64(value) => query = query.bind(value),
        }
    }

    query.fetch_all(&mut *conn).await
}
// ==================== UPDATE ====================

pub trait UpdateData<E: Entity> {
    fn columns(&self) -> Vec<&'static str>;
    fn values(&self) -> Vec<BindValue>;
}

pub async fn update<'c, E, D, A>(db: A, id: i32, data: D) -> Result<E::Model, sqlx::Error>
where
    E: Entity,
    D: UpdateData<E>,
    A: sqlx::Acquire<'c, Database = sqlx::Postgres>,
    for<'r> E::Model: sqlx::FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
{
    let mut conn = db.acquire().await?;
    let columns = data.columns();
    let values = data.values();

    if columns.is_empty() {
        return Err(sqlx::Error::Protocol(
            "UPDATE requires at least one field".into(),
        ));
    }

    let set_clause = columns
        .iter()
        .enumerate()
        .map(|(index, column)| format!("{} = ${}", column, index + 1))
        .collect::<Vec<_>>()
        .join(", ");

    let returning = E::columns()
        .iter()
        .map(|column| column.name())
        .collect::<Vec<_>>()
        .join(", ");

    let sql = format!(
        "UPDATE {} SET {} WHERE id = ${} RETURNING {}",
        E::TABLE,
        set_clause,
        values.len() + 1,
        returning
    );

    let mut query = sqlx::query_as::<_, E::Model>(&sql);

    for value in values {
        match value {
            BindValue::String(value) => query = query.bind(value),
            BindValue::I64(value) => query = query.bind(value),
        }
    }

    query = query.bind(id);

    query.fetch_one(&mut *conn).await
}

pub async fn delete<'c, E, A>(db: A, id: i32) -> Result<u64, sqlx::Error>
where
    E: Entity,
    A: sqlx::Acquire<'c, Database = sqlx::Postgres>,
{
    let mut conn = db.acquire().await?;

    let sql = format!("DELETE FROM {} WHERE id = $1", E::TABLE);

    let result = sqlx::query(&sql).bind(id).execute(&mut *conn).await?;

    Ok(result.rows_affected())
}

pub async fn update_many<'c, E, D, A>(
    db: A,
    condition: Condition<E>,
    data: D,
) -> Result<u64, sqlx::Error>
where
    E: Entity,
    D: UpdateData<E>,
    A: sqlx::Acquire<'c, Database = sqlx::Postgres>,
{
    let mut conn = db.acquire().await?;

    let columns = data.columns();
    let values = data.values();

    if columns.is_empty() {
        return Err(sqlx::Error::Protocol(
            "UPDATE requires at least one field".into(),
        ));
    }

    let mut next_placeholder = 1;

    let set_clause = columns
        .iter()
        .map(|column| {
            let placeholder = format!("${}", next_placeholder);
            next_placeholder += 1;

            format!("{} = {}", column, placeholder)
        })
        .collect::<Vec<_>>()
        .join(", ");

    let condition_expression = condition.into_ast();

    let where_clause = compile_expression(&condition_expression, &mut next_placeholder);

    let sql = format!(
        "UPDATE {} SET {} WHERE {}",
        E::TABLE,
        set_clause,
        where_clause
    );

    let mut binds = values;

    crate::sql::collect_bind_values(&condition_expression, &mut binds);

    let mut query = sqlx::query(&sql);

    for value in binds {
        match value {
            BindValue::String(value) => {
                query = query.bind(value);
            }
            BindValue::I64(value) => {
                query = query.bind(value);
            }
        }
    }

    let result = query.execute(&mut *conn).await?;

    Ok(result.rows_affected())
}
pub async fn delete_many<'c, E, A>(db: A, condition: Condition<E>) -> Result<u64, sqlx::Error>
where
    E: Entity,
    A: sqlx::Acquire<'c, Database = sqlx::Postgres>,
{
    let mut conn = db.acquire().await?;
    let condition_expression = condition.into_ast();

    let mut next_placeholder = 1;

    let where_clause = compile_expression(&condition_expression, &mut next_placeholder);

    let sql = format!("DELETE FROM {} WHERE {}", E::TABLE, where_clause);

    let mut binds = Vec::new();

    crate::sql::collect_bind_values(&condition_expression, &mut binds);

    let mut query = sqlx::query(&sql);

    for value in binds {
        match value {
            BindValue::String(value) => {
                query = query.bind(value);
            }
            BindValue::I64(value) => {
                query = query.bind(value);
            }
        }
    }

    let result = query.execute(&mut *conn).await?;

    Ok(result.rows_affected())
}
