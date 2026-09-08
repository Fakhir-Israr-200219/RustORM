use std::marker::PhantomData;

use crate::entity::{Column, Entity};
use crate::query::condition::Condition;
use crate::query::expression::Expression;
use crate::query::statement::{
    OrderBy,
    OrderDirection,
    SelectItem,
    SelectStatement,
};
use crate::value::BindValue;
use crate::sql::{
    collect_bind_values,
    compile_expression,
};

pub struct Query<E> {
    pub(crate) statement: SelectStatement<E>,
    _entity: PhantomData<E>,
}


impl<E> Query<E>
where
    E: Entity,
{
    pub(crate) fn new() -> Self {
        Self {
            statement: SelectStatement {
                columns: E::columns()
                    .iter()
                    .map(|column| SelectItem {
                        expression: Expression::Column(*column),
                        _entity: PhantomData,
                    })
                    .collect(),

                table: E::TABLE,
                where_clause: None,
                order_by: None,
                limit: None,
                offset: None,
                distinct: false,
                group_by: Vec::new(),
                having: None,
                selected_explicitly: false,
            },
            _entity: PhantomData,
        }
    }

    pub fn where_(mut self, condition: Condition<E>) -> Self {
        self.statement.where_clause = Some(condition.into_ast());
        self
    }

    pub fn order_by(mut self, order: OrderBy) -> Self {
        self.statement.order_by = Some(order);
        self
    }

    pub fn take(mut self, limit: u64) -> Self {
        self.statement.limit = Some(limit);
        self
    }

    pub fn skip(mut self, offset: u64) -> Self {
        self.statement.offset = Some(offset);
        self
    }

    pub fn distinct(mut self) -> Self {
        self.statement.distinct = true;
        self
    }

    pub fn group_by(mut self, column: Column) -> Self {
        self.statement.group_by.push(column);
        self
    }

    pub fn having(mut self, condition: Condition<E>) -> Self {
        self.statement.having = Some(condition.expression);
        self
    }

    pub fn select(mut self, item: SelectItem<E>) -> Self {
        if !self.statement.selected_explicitly {
            self.statement.columns.clear();
            self.statement.selected_explicitly = true;
        }

        self.statement.columns.push(item);

        self
    }
}

//
// SQL generation
//

impl<E> Query<E>
where
    E: Entity,
{
    pub(crate) fn build_sql(&self) -> String {
        let mut next_placeholder = 1;
        let columns = self
            .statement
            .columns
            .iter()
            .map(|item| compile_expression(&item.expression, &mut next_placeholder))
            .collect::<Vec<_>>()
            .join(", ");

        let mut sql = if self.statement.distinct {
            format!("SELECT DISTINCT {} FROM {}", columns, self.statement.table)
        } else {
            format!("SELECT {} FROM {}", columns, self.statement.table)
        };

        if let Some(condition) = &self.statement.where_clause {
            sql.push_str(" WHERE ");
            sql.push_str(&compile_expression(condition, &mut next_placeholder));
        }

        if !self.statement.group_by.is_empty() {
            sql.push_str(" GROUP BY ");

            let columns = self
                .statement
                .group_by
                .iter()
                .map(|column| column.name())
                .collect::<Vec<_>>()
                .join(", ");

            sql.push_str(&columns);
        }
        if let Some(condition) = &self.statement.having {
            sql.push_str(" HAVING ");
            sql.push_str(&compile_expression(condition, &mut next_placeholder));
        }
        if let Some(order_by) = &self.statement.order_by {
            sql.push_str(" ORDER BY ");
            sql.push_str(order_by.column.name());

            match order_by.direction {
                OrderDirection::Asc => {
                    sql.push_str(" ASC");
                }

                OrderDirection::Desc => {
                    sql.push_str(" DESC");
                }
            }
        }

        if let Some(limit) = self.statement.limit {
            sql.push_str(&format!(" LIMIT {}", limit));
        }

        if let Some(offset) = self.statement.offset {
            sql.push_str(&format!(" OFFSET {}", offset));
        }

        sql
    }
}

impl<E> Query<E>
where
    E: Entity,
    for<'r> E::Model: sqlx::FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
{
    pub async fn all(self, db: &sqlx::PgPool) -> Result<Vec<E::Model>, sqlx::Error> {
        let sql = self.build_sql();
        let mut query = sqlx::query_as::<_, E::Model>(&sql);

        if let Some(expression) = &self.statement.where_clause {
            let mut values = Vec::new();

            collect_bind_values(expression, &mut values);

            for value in values {
                match value {
                    BindValue::String(value) => {
                        query = query.bind(value);
                    }

                    BindValue::I64(value) => {
                        query = query.bind(value);
                    }
                }
            }
        }

        if let Some(expression) = &self.statement.having {
            let mut values = Vec::new();

            collect_bind_values(expression, &mut values);

            for value in values {
                match value {
                    BindValue::String(value) => {
                        query = query.bind(value);
                    }

                    BindValue::I64(value) => {
                        query = query.bind(value);
                    }
                }
            }
        }

        query.fetch_all(db).await
    }
    pub async fn count(self, db: &sqlx::PgPool) -> Result<i64, sqlx::Error> {
        let sql = self.build_sql();

        let mut query = sqlx::query_scalar::<_, i64>(&sql);

        if let Some(expression) = &self.statement.where_clause {
            let mut values = Vec::new();

            collect_bind_values(expression, &mut values);

            for value in values {
                match value {
                    BindValue::String(value) => {
                        query = query.bind(value);
                    }

                    BindValue::I64(value) => {
                        query = query.bind(value);
                    }
                }
            }
        }

        if let Some(expression) = &self.statement.having {
            let mut values = Vec::new();

            collect_bind_values(expression, &mut values);

            for value in values {
                match value {
                    BindValue::String(value) => {
                        query = query.bind(value);
                    }

                    BindValue::I64(value) => {
                        query = query.bind(value);
                    }
                }
            }
        }

        query.fetch_one(db).await
    }
    pub async fn fetch_all<R>(self, db: &sqlx::PgPool) -> Result<Vec<R>, sqlx::Error>
    where
        for<'r> R: sqlx::FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
    {
        let sql = self.build_sql();

        let mut query = sqlx::query_as::<_, R>(&sql);

        if let Some(expression) = &self.statement.where_clause {
            let mut values = Vec::new();

            collect_bind_values(expression, &mut values);

            for value in values {
                match value {
                    BindValue::String(value) => {
                        query = query.bind(value);
                    }

                    BindValue::I64(value) => {
                        query = query.bind(value);
                    }
                }
            }
        }

        if let Some(expression) = &self.statement.having {
            let mut values = Vec::new();

            collect_bind_values(expression, &mut values);

            for value in values {
                match value {
                    BindValue::String(value) => {
                        query = query.bind(value);
                    }

                    BindValue::I64(value) => {
                        query = query.bind(value);
                    }
                }
            }
        }

        query.fetch_all(db).await
    }
}