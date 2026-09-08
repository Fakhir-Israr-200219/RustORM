use crate::entity::Column;
use crate::value::BindValue;

pub(crate) enum BinaryOperator {
    Eq,
    NotEq,
    Gt,
    Gte,
    Lt,
    Lte,
    In,
    And,
    Or,
}

pub(crate) enum AggregateFunction {
    Count,
    Sum,
    Avg,
    Min,
    Max,
}
pub(crate) struct Subquery {
    pub(crate) sql: String,
}

pub(crate) enum Expression {
    Column(Column),
    Value(BindValue),

    Function {
        function: AggregateFunction,
        expression: Box<Expression>,
    },

    Binary {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    },

    Subquery(Subquery),
}