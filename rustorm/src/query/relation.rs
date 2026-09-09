use std::collections::HashMap;
use std::marker::PhantomData;

use sqlx::postgres::PgRow;

use crate::entity::{Column, Entity, RelationKey, RelationLoader};
use crate::field::Field;
use crate::query::Query;
use crate::query::join::{JoinCondition, JoinTarget};
use crate::query::query::{NoRelations, RelationNode};

pub trait RelationLoad<E>
where
    E: Entity,
{
    fn load(
        &self,
        db: &sqlx::PgPool,
        parents: &mut [E::Model],
    ) -> impl std::future::Future<Output = Result<(), sqlx::Error>>;
}

impl<E> RelationLoad<E> for NoRelations
where
    E: Entity,
{
    fn load(
        &self,
        _db: &sqlx::PgPool,
        _parents: &mut [E::Model],
    ) -> impl std::future::Future<Output = Result<(), sqlx::Error>> {
        async { Ok(()) }
    }
}

impl<From, To, Tail> RelationLoad<From> for RelationNode<Relation<From, To>, Tail>
where
    From: Entity,
    To: Entity,
    From::Model: RelationKey + RelationLoader<To::Model>,
    To::Model: RelationKey + for<'r> sqlx::FromRow<'r, PgRow> + Send + Unpin,
    Tail: RelationLoad<From>,
{
    async fn load(
        &self,
        db: &sqlx::PgPool,
        parents: &mut [From::Model],
    ) -> Result<(), sqlx::Error> {
        if parents.is_empty() {
            return self.tail.load(db, parents).await;
        }

        let values = self.relation.parent_key_values(parents);

        let child_query = self
            .relation
            .target_query()
            .apply_relation_filter(&self.relation.info(), values);

        let children = child_query.all(db).await?;

        let mut grouped: HashMap<i64, Vec<To::Model>> = HashMap::new();

        for child in children {
            if let Some(key) = self.relation.child_key(&child) {
                grouped.entry(key).or_default().push(child);
            }
        }

        for parent in parents.iter_mut() {
            if let Some(key) = self.relation.parent_key(parent) {
                let related = grouped.remove(&key).unwrap_or_default();

                self.relation.attach(parent, related);
            }
        }

        self.tail.load(db, parents).await
    }
}

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

    pub(crate) fn info(&self) -> crate::query::statement::RelationInfo {
        crate::query::statement::RelationInfo {
            // from_table: From::TABLE,
            // from_column: self.from,
            to_table: To::TABLE,
            to_column: self.to,
        }
    }

    pub(crate) fn target_query(&self) -> Query<To> {
        Query::<To>::new()
    }

    pub fn attach(&self, parent: &mut From::Model, related: Vec<To::Model>)
    where
        From::Model: RelationLoader<To::Model>,
    {
        parent.load_relation(related);
    }

    pub fn parent_key(&self, parent: &From::Model) -> Option<i64>
    where
        From::Model: RelationKey,
    {
        parent.relation_key(self.from)
    }

    pub fn child_key(&self, child: &To::Model) -> Option<i64>
    where
        To::Model: RelationKey,
    {
        child.relation_key(self.to)
    }

    pub fn parent_keys(&self, parents: &[From::Model]) -> Vec<i64>
    where
        From::Model: RelationKey,
    {
        parents
            .iter()
            .filter_map(|parent| self.parent_key(parent))
            .collect()
    }

    pub(crate) fn parent_key_values(&self, parents: &[From::Model]) -> Vec<crate::value::BindValue>
    where
        From::Model: RelationKey,
    {
        self.parent_keys(parents)
            .into_iter()
            .map(crate::value::BindValue::I64)
            .collect()
    }

    // pub(crate) fn child_filter(
    //     &self,
    //     parents: &[From::Model],
    // ) -> crate::query::condition::Condition<To>
    // where
    //     From::Model: RelationKey,
    // {
    //     let values = self.parent_key_values(parents);

    //     crate::query::condition::Condition {
    //         expression: self.info().foreign_key_in(values),
    //         _entity: PhantomData,
    //     }
    // }
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
