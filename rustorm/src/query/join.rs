use crate::query::expression::Expression;

pub struct JoinCondition {
    pub(crate) table: &'static str,
    pub(crate) expression: Expression,
}

impl JoinCondition {
    pub(crate) fn into_parts(self) -> (&'static str, Expression) {
        (self.table, self.expression)
    }
}

pub trait JoinTarget {
    fn into_join_condition(self) -> JoinCondition;
}

impl JoinTarget for JoinCondition {
    fn into_join_condition(self) -> JoinCondition {
        self
    }
}

pub(crate) enum JoinType {
    Inner,
    Left,
    Right,
    Full,
    Cross,
}

pub(crate) struct Join {
    pub(crate) join_type: JoinType,
    pub(crate) table: &'static str,
    pub(crate) on: Option<Expression>,
}