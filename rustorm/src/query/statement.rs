use std::marker::PhantomData;

use crate::entity::Column;
use crate::query::condition::Condition;
use crate::query::expression::{BinaryOperator, Expression};
use crate::query::join::Join;
use crate::value::BindValue;

pub struct SelectStatement<E> {
    pub(crate) columns: Vec<SelectItem<E>>,
    pub(crate) table: &'static str,
    pub(crate) joins: Vec<Join>,
    pub(crate) where_clause: Option<Expression>,
    pub(crate) order_by: Option<OrderBy>,
    pub(crate) limit: Option<u64>,
    pub(crate) offset: Option<u64>,
    pub(crate) distinct: bool,
    pub(crate) group_by: Vec<Column>,
    pub(crate) having: Option<Expression>,
    pub(crate) selected_explicitly: bool,
    pub(crate) relations: Vec<RelationInfo>,
}

pub(crate) struct RelationInfo {
    // pub(crate) from_table: &'static str,
    // pub(crate) from_column: Column,
    pub(crate) to_table: &'static str,
    pub(crate) to_column: Column,
}

impl RelationInfo {
    pub(crate) fn to_table(&self) -> &'static str {
        self.to_table
    }

    pub(crate) fn to_column(&self) -> Column {
        self.to_column
    }

    pub(crate) fn foreign_key_in(&self, values: Vec<crate::value::BindValue>) -> Expression {
        Expression::Binary {
            left: Box::new(Expression::Column(Column::qualified(
                self.to_table(),
                self.to_column().name(),
            ))),
            operator: BinaryOperator::In,
            right: Box::new(Expression::List(
                values.into_iter().map(Expression::Value).collect(),
            )),
        }
    }
}

pub enum OrderDirection {
    Asc,
    Desc,
}

pub struct OrderBy {
    pub(crate) column: Column,
    pub(crate) direction: OrderDirection,
}

pub struct SelectItem<E> {
    pub(crate) expression: Expression,
    pub(crate) _entity: PhantomData<E>,
}

impl<E> SelectItem<E> {
    pub fn gt(self, value: i32) -> Condition<E> {
        Condition {
            expression: Expression::Binary {
                left: Box::new(self.expression),
                operator: BinaryOperator::Gt,
                right: Box::new(Expression::Value(BindValue::I64(value as i64))),
            },
            _entity: PhantomData,
        }
    }
}

