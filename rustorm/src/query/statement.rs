use std::marker::PhantomData;

use crate::entity::Column;
use crate::query::condition::Condition;
use crate::query::expression::{
    BinaryOperator,
    Expression,
};
use crate::value::BindValue;
use crate::query::join::Join;

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
