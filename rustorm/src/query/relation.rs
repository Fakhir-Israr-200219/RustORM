use sqlx::Row;
use sqlx::postgres::PgRow;
use std::collections::{HashMap, HashSet};
use std::marker::PhantomData;
use std::sync::Arc;

use crate::entity::{Column, Entity, RelationKey, RelationLoader, SingleRelationLoader};
use crate::field::Field;
use crate::query::Query;
use crate::query::join::{JoinCondition, JoinTarget};
use crate::query::query::{NoRelations, RelationNode};

pub struct OneToMany;
pub struct ManyToOne;
pub struct OneToOne;
pub struct ManyToMany;

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

impl<From, To, Tail> RelationLoad<From> for RelationNode<Relation<From, To, OneToMany>, Tail>
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

impl<From, To, Tail> RelationLoad<From> for RelationNode<Relation<From, To, ManyToOne>, Tail>
where
    From: Entity,
    To: Entity,
    From::Model: RelationKey + SingleRelationLoader<Arc<To::Model>>,
    To::Model: RelationKey + for<'r> sqlx::FromRow<'r, PgRow> + Send + Sync + Unpin,
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

        let mut grouped: HashMap<i64, Arc<To::Model>> = HashMap::new();

        for child in children {
            if let Some(key) = self.relation.child_key(&child) {
                grouped.insert(key, Arc::new(child));
            }
        }

        for parent in parents.iter_mut() {
            let related = self
                .relation
                .parent_key(parent)
                .and_then(|key| grouped.get(&key).cloned());

            self.relation.attach_single(parent, related);
        }

        self.tail.load(db, parents).await
    }
}
impl<From, To, Tail> RelationLoad<From> for RelationNode<Relation<From, To, OneToOne>, Tail>
where
    From: Entity,
    To: Entity,
    From::Model: RelationKey + SingleRelationLoader<Arc<To::Model>>,
    To::Model: RelationKey + for<'r> sqlx::FromRow<'r, PgRow> + Send + Sync + Unpin,
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

        let mut grouped: HashMap<i64, Arc<To::Model>> = HashMap::new();

        for child in children {
            if let Some(key) = self.relation.child_key(&child) {
                grouped.insert(key, Arc::new(child));
            }
        }

        for parent in parents.iter_mut() {
            let related = self
                .relation
                .parent_key(parent)
                .and_then(|key| grouped.get(&key).cloned());

            self.relation.attach_single(parent, related);
        }

        self.tail.load(db, parents).await
    }
}
impl<From, To, Tail> RelationLoad<From> for RelationNode<Relation<From, To, ManyToMany>, Tail>
where
    From: Entity,
    To: Entity,
    From::Model: RelationKey + RelationLoader<Arc<To::Model>>,
    To::Model: RelationKey + for<'r> sqlx::FromRow<'r, PgRow> + Send + Sync + Unpin,
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

        let pivot_table = self
            .relation
            .pivot_table
            .expect("ManyToMany relation requires pivot table");

        let pivot_from = self
            .relation
            .pivot_from
            .expect("ManyToMany relation requires pivot from column");

        let pivot_to = self
            .relation
            .pivot_to
            .expect("ManyToMany relation requires pivot to column");

        let parent_keys = self.relation.parent_keys(parents);

        if parent_keys.is_empty() {
            return self.tail.load(db, parents).await;
        }

        let placeholders = (1..=parent_keys.len())
            .map(|i| format!("${i}"))
            .collect::<Vec<_>>()
            .join(", ");

        let sql = format!(
            "SELECT {}::BIGINT AS from_id, {}::BIGINT AS to_id FROM {} WHERE {} IN ({})",
            pivot_from.name(),
            pivot_to.name(),
            pivot_table,
            pivot_from.name(),
            placeholders
        );

        let mut query = sqlx::query(&sql);

        for key in &parent_keys {
            query = query.bind(*key);
        }

        let rows = query.fetch_all(db).await?;

        let mut parent_to_children: HashMap<i64, Vec<i64>> = HashMap::new();
        let mut target_ids = HashSet::new();

        for row in rows {
            let from_id: i64 = row.try_get("from_id")?;
            let to_id: i64 = row.try_get("to_id")?;

            parent_to_children.entry(from_id).or_default().push(to_id);

            target_ids.insert(to_id);
        }

        if target_ids.is_empty() {
            for parent in parents.iter_mut() {
                self.relation.attach_shared(parent, Vec::new());
            }

            return self.tail.load(db, parents).await;
        }

        let values = target_ids
            .into_iter()
            .map(crate::value::BindValue::I64)
            .collect();

        let child_query = self
            .relation
            .target_query()
            .apply_relation_filter(&self.relation.info(), values);

        let children = child_query.all(db).await?;

        let mut grouped: HashMap<i64, Arc<To::Model>> = HashMap::new();

        for child in children {
            if let Some(key) = self.relation.child_key(&child) {
                grouped.insert(key, Arc::new(child));
            }
        }

        for parent in parents.iter_mut() {
            let related = self
                .relation
                .parent_key(parent)
                .and_then(|key| parent_to_children.get(&key))
                .into_iter()
                .flat_map(|ids| ids.iter())
                .filter_map(|id| grouped.get(id).cloned())
                .collect();

            self.relation.attach_shared(parent, related);
        }

        self.tail.load(db, parents).await
    }
}

pub struct Relation<From, To, C = OneToMany>
where
    From: Entity,
    To: Entity,
{
    from: Column,
    to: Column,

    pivot_table: Option<&'static str>,
    pivot_from: Option<Column>,
    pivot_to: Option<Column>,

    _from: PhantomData<From>,
    _to: PhantomData<To>,
    _cardinality: PhantomData<C>,
}

impl<From, To, C> Relation<From, To, C>
where
    From: Entity,
    To: Entity,
{
    pub const fn new<T>(from: Field<From, T>, to: Field<To, T>) -> Self {
        Self {
            from: from.column(),
            to: to.column(),

            pivot_table: None,
            pivot_from: None,
            pivot_to: None,

            _from: PhantomData,
            _to: PhantomData,
            _cardinality: PhantomData,
        }
    }

    pub(crate) fn info(&self) -> crate::query::statement::RelationInfo {
        crate::query::statement::RelationInfo {
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

    pub fn attach_single(&self, parent: &mut From::Model, related: Option<Arc<To::Model>>)
    where
        From::Model: SingleRelationLoader<Arc<To::Model>>,
    {
        parent.load_relation(related);
    }
    pub fn attach_shared(&self, parent: &mut From::Model, related: Vec<Arc<To::Model>>)
    where
        From::Model: RelationLoader<Arc<To::Model>>,
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
    pub const fn many_to_many<T>(
        from: Field<From, T>,
        to: Field<To, T>,
        pivot_table: &'static str,
        pivot_from: Column,
        pivot_to: Column,
    ) -> Relation<From, To, ManyToMany> {
        Relation {
            from: from.column(),
            to: to.column(),

            pivot_table: Some(pivot_table),
            pivot_from: Some(pivot_from),
            pivot_to: Some(pivot_to),

            _from: PhantomData,
            _to: PhantomData,
            _cardinality: PhantomData,
        }
    }
}

impl<From, To, C> JoinTarget for Relation<From, To, C>
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
