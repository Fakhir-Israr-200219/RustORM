use std::marker::PhantomData;

use crate::value::BindValue;
use crate::Column;
use crate::OrderBy;
use crate::OrderDirection;
use crate::SelectItem;
use crate::Expression;
use crate::AggregateFunction;
use crate::BinaryOperator;
use crate::Condition;

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

    pub fn column(&self) -> Column {
        self.column
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
}

impl<E> Field<E, String> {
    fn compare(
        &self,
        operator: BinaryOperator,
        value: impl Into<String>,
    ) -> Condition<E> {
        Condition {
            expression: Expression::Binary {
                left: Box::new(Expression::Column(self.column)),
                operator,
                right: Box::new(Expression::Value(
                    BindValue::String(value.into())
                )),
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
    fn compare(
        &self,
        operator: BinaryOperator,
        value: i32,
    ) -> Condition<E> {
        Condition {
            expression: Expression::Binary {
                left: Box::new(Expression::Column(self.column)),
                operator,
                right: Box::new(Expression::Value(
                    BindValue::I64(value as i64)
                )),
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