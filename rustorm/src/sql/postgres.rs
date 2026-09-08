use crate::{
    query::expression::{AggregateFunction, BinaryOperator, Expression},
    value::BindValue,
};

pub(crate) fn compile_expression(expression: &Expression, next_placeholder: &mut usize) -> String {
    match expression {
        Expression::Column(column) => match column.table() {
            Some(table) => format!("{}.{}", table, column.name()),
            None => column.name().to_string(),
        },

        Expression::Value(_) => {
            let placeholder = format!("${}", *next_placeholder);
            *next_placeholder += 1;
            placeholder
        }

        Expression::Binary {
            left,
            operator,
            right,
        } => {
            let left_sql = compile_expression(left, next_placeholder);

            let operator_sql = match operator {
                BinaryOperator::Eq => "=",
                BinaryOperator::NotEq => "<>",
                BinaryOperator::Gt => ">",
                BinaryOperator::Gte => ">=",
                BinaryOperator::Lt => "<",
                BinaryOperator::Lte => "<=",
                BinaryOperator::And => "AND",
                BinaryOperator::Or => "OR",
                BinaryOperator::In => "IN",
            };

            let right_sql = compile_expression(right, next_placeholder);

            format!("{} {} {}", left_sql, operator_sql, right_sql)
        }
        Expression::Function {
            function,
            expression,
        } => {
            let expression_sql = compile_expression(expression, next_placeholder);

            match function {
                AggregateFunction::Count => {
                    format!("COUNT({})", expression_sql)
                }

                AggregateFunction::Sum => {
                    format!("SUM({})", expression_sql)
                }

                AggregateFunction::Avg => {
                    format!("AVG({})", expression_sql)
                }
                AggregateFunction::Min => {
                    format!("MIN({})", expression_sql)
                }
                AggregateFunction::Max => {
                    format!("MAX({})", expression_sql)
                }
            }
        }
        Expression::Subquery(subquery) => {
            format!("({})", subquery.sql)
        }
    }
}

pub(crate) fn collect_bind_values(expression: &Expression, values: &mut Vec<BindValue>) {
    match expression {
        Expression::Column(_) => {}

        Expression::Value(value) => match value {
            BindValue::String(value) => {
                values.push(BindValue::String(value.clone()));
            }

            BindValue::I64(value) => {
                values.push(BindValue::I64(*value));
            }
        },
        Expression::Function { expression, .. } => {
            collect_bind_values(expression, values);
        }
        Expression::Binary { left, right, .. } => {
            collect_bind_values(left, values);
            collect_bind_values(right, values);
        }
        Expression::Subquery(_) => {}
    }
}
