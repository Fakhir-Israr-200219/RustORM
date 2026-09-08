use std::marker::PhantomData;

use crate::AggregateFunction;
use crate::BinaryOperator;
use crate::Column;
use crate::Condition;
use crate::Expression;
use crate::OrderBy;
use crate::OrderDirection;
use crate::SelectItem;
use crate::entity::Entity;
use crate::query::expression::Subquery;
use crate::query::join::JoinCondition;
use crate::value::BindValue;

pub struct Field<E, T> {
    column: Column,
    _entity: PhantomData<E>,
    _type: PhantomData<T>,
}

impl<E, T> Field<E, T> {
    pub const fn new(name: &'static str) -> Self {
        Self {
            column: Column::new(name),
            _entity: PhantomData,
            _type: PhantomData,
        }
    }

    pub const fn name(&self) -> &'static str {
        self.column.name()
    }

    pub fn asc(&self) -> OrderBy {
        OrderBy {
            column: self.column,
            direction: OrderDirection::Asc,
        }
    }

    pub fn desc(&self) -> OrderBy {
        OrderBy {
            column: self.column,
            direction: OrderDirection::Desc,
        }
    }

    pub fn count(&self) -> SelectItem<E> {
        SelectItem {
            expression: Expression::Function {
                function: AggregateFunction::Count,
                expression: Box::new(Expression::Column(self.column)),
            },
            _entity: PhantomData,
        }
    }

    pub fn select(&self) -> SelectItem<E> {
        SelectItem {
            expression: Expression::Column(self.column),
            _entity: PhantomData,
        }
    }

    pub const fn column(&self) -> Column {
        self.column
    }

    pub fn eq_column<E2>(&self, other: Field<E2, T>) -> JoinCondition
    where
        E: Entity,
        E2: Entity,
    {
        JoinCondition {
            table: E::TABLE,
            expression: Expression::Binary {
                left: Box::new(Expression::Column(Column::qualified(
                    E::TABLE,
                    self.column.name(),
                ))),
                operator: BinaryOperator::Eq,
                right: Box::new(Expression::Column(Column::qualified(
                    E2::TABLE,
                    other.column.name(),
                ))),
            },
        }
    }

    pub fn sum(&self) -> SelectItem<E> {
        SelectItem {
            expression: Expression::Function {
                function: AggregateFunction::Sum,
                expression: Box::new(Expression::Column(self.column)),
            },
            _entity: PhantomData,
        }
    }

    pub fn avg(&self) -> SelectItem<E> {
        SelectItem {
            expression: Expression::Function {
                function: AggregateFunction::Avg,
                expression: Box::new(Expression::Column(self.column)),
            },
            _entity: PhantomData,
        }
    }

    pub fn min(&self) -> SelectItem<E> {
        SelectItem {
            expression: Expression::Function {
                function: AggregateFunction::Min,
                expression: Box::new(Expression::Column(self.column)),
            },
            _entity: PhantomData,
        }
    }

    pub fn max(&self) -> SelectItem<E> {
        SelectItem {
            expression: Expression::Function {
                function: AggregateFunction::Max,
                expression: Box::new(Expression::Column(self.column)),
            },
            _entity: PhantomData,
        }
    }
    pub fn in_subquery<E2>(&self, subquery: crate::query::Query<E2>) -> Condition<E>
    where
        E: Entity,
        E2: Entity,
    {
        Condition {
            expression: Expression::Binary {
                left: Box::new(Expression::Column(self.column)),
                operator: BinaryOperator::In,
                right: Box::new(Expression::Subquery(Subquery {
                    sql: subquery.build_sql(),
                })),
            },
            _entity: PhantomData,
        }
    }
}

impl<E> Field<E, String> {
    fn compare(&self, operator: BinaryOperator, value: impl Into<String>) -> Condition<E> {
        Condition {
            expression: Expression::Binary {
                left: Box::new(Expression::Column(self.column)),
                operator,
                right: Box::new(Expression::Value(BindValue::String(value.into()))),
            },
            _entity: PhantomData,
        }
    }

    pub fn eq(&self, value: impl Into<String>) -> Condition<E> {
        self.compare(BinaryOperator::Eq, value)
    }

    pub fn not_eq(&self, value: impl Into<String>) -> Condition<E> {
        self.compare(BinaryOperator::NotEq, value)
    }

    pub fn gt(&self, value: impl Into<String>) -> Condition<E> {
        self.compare(BinaryOperator::Gt, value)
    }

    pub fn gte(&self, value: impl Into<String>) -> Condition<E> {
        self.compare(BinaryOperator::Gte, value)
    }

    pub fn lt(&self, value: impl Into<String>) -> Condition<E> {
        self.compare(BinaryOperator::Lt, value)
    }

    pub fn lte(&self, value: impl Into<String>) -> Condition<E> {
        self.compare(BinaryOperator::Lte, value)
    }
}

impl<E> Field<E, i32> {
    fn compare(&self, operator: BinaryOperator, value: i32) -> Condition<E> {
        Condition {
            expression: Expression::Binary {
                left: Box::new(Expression::Column(self.column)),
                operator,
                right: Box::new(Expression::Value(BindValue::I64(value as i64))),
            },
            _entity: PhantomData,
        }
    }

    pub fn eq(&self, value: i32) -> Condition<E> {
        self.compare(BinaryOperator::Eq, value)
    }

    pub fn not_eq(&self, value: i32) -> Condition<E> {
        self.compare(BinaryOperator::NotEq, value)
    }

    pub fn gt(&self, value: i32) -> Condition<E> {
        self.compare(BinaryOperator::Gt, value)
    }

    pub fn gte(&self, value: i32) -> Condition<E> {
        self.compare(BinaryOperator::Gte, value)
    }

    pub fn lt(&self, value: i32) -> Condition<E> {
        self.compare(BinaryOperator::Lt, value)
    }

    pub fn lte(&self, value: i32) -> Condition<E> {
        self.compare(BinaryOperator::Lte, value)
    }
}
