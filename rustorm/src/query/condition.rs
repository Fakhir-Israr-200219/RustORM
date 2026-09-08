use std::marker::PhantomData;
use crate::query::expression::{BinaryOperator, Expression};

pub struct Condition<E> {
    pub(crate) expression: Expression,
    pub(crate) _entity: PhantomData<E>,
}


impl<E> Condition<E> {
    pub fn and(self, other: Condition<E>) -> Condition<E> {
        Condition {
            expression: Expression::Binary {
                left: Box::new(self.expression),
                operator: BinaryOperator::And,
                right: Box::new(other.expression),
            },
            _entity: PhantomData,
        }
    }

    pub fn or(self, other: Condition<E>) -> Condition<E> {
        Condition {
            expression: Expression::Binary {
                left: Box::new(self.expression),
                operator: BinaryOperator::Or,
                right: Box::new(other.expression),
            },
            _entity: PhantomData,
        }
    }

    pub(crate) fn into_ast(self) -> Expression {
        self.expression
    }
}
