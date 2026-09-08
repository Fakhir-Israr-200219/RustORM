use crate::entity::Column;
use crate::value::BindValue;

pub enum BinaryOperator {
    Eq,
    NotEq,
    Gt,
    Gte,
    Lt,
    Lte,
    And,
    Or,
}

pub enum AggregateFunction {
    Count,
    Sum,
    Avg,
    Min,
    Max,
}

pub enum Expression {
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
}