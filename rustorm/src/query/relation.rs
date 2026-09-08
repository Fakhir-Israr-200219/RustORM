use std::marker::PhantomData;

use crate::entity::{Column, Entity};
use crate::field::Field;
use crate::query::join::{JoinCondition, JoinTarget};

pub struct Relation<From, To>
where
    From: Entity,
    To: Entity,
{
    from: Column,
    to: Column,
    _from: PhantomData<From>,
    _to: PhantomData<To>,
}

impl<From, To> Relation<From, To>
where
    From: Entity,
    To: Entity,
{
    pub const fn new<T>(from: Field<From, T>, to: Field<To, T>) -> Self {
        Self {
            from: from.column(),
            to: to.column(),
            _from: PhantomData,
            _to: PhantomData,
        }
    }
}

impl<From, To> JoinTarget for Relation<From, To>
where
    From: Entity,
    To: Entity,
{
    fn into_join_condition(self) -> JoinCondition {
        JoinCondition {
            table: To::TABLE,
            expression: crate::query::expression::Expression::Binary {
                left: Box::new(crate::query::expression::Expression::Column(
                    Column::qualified(From::TABLE, self.from.name()),
                )),
                operator: crate::query::expression::BinaryOperator::Eq,
                right: Box::new(crate::query::expression::Expression::Column(
                    Column::qualified(To::TABLE, self.to.name()),
                )),
            },
        }
    }
}
