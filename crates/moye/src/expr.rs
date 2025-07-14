mod block;
mod binding_usage;
mod func_call;

pub(crate) use binding_usage::BindingUsage;
pub(crate) use block::Block;
pub(crate) use func_call::FuncCall;

use crate::env::Env;
use crate::utils;
use crate::val::Val;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Number(pub i32);

impl Number {
    fn new(s:&str) -> Result<(&str, Self), String> {
        let (s, number) = utils::extract_digits(s)?;
        Ok((s, Self(number.parse().unwrap())))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Operations {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operations {
    fn new(s:&str) -> Result<(&str, Self), String> {
        utils::tag("+", s)
            .map(|s| (s, Self::Add))
            .or_else(|_| utils::tag("-", s).map(|s| (s, Self::Sub)))
            .or_else(|_| utils::tag("*", s).map(|s| (s, Self::Mul)))
            .or_else(|_| utils::tag("/", s).map(|s| (s, Self::Div)))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Expression {
    Number(Number),
    Operation { lhs: Box<Self>, rhs: Box<Self>, op: Operations },
    FuncCall(FuncCall),
    BindingUsage(BindingUsage),
    Block(Block),
}

impl Expression {
    pub(crate) fn new(s:&str) -> Result<(&str, Self), String> {
        Self::new_operation(s).or_else(|_| Self::new_non_operation(s))
    }

    fn new_non_operation(s:&str) -> Result<(&str, Self), String> {
        Self::new_number(s)
            .or_else(|_| FuncCall::new(s).map(|(s, func_call)| (s, Self::FuncCall(func_call))))
            .or_else(|_| {
                BindingUsage::new(s)
                    .map(|(s, binding_usage)| (s, Self::BindingUsage(binding_usage)))
            })
            .or_else(|_| Block::new(s).map(|(s, block)| (s, Self::Block(block))))
    }

    fn new_operation(s:&str) -> Result<(&str, Self), String> {
        let (s, lhs) = Self::new_non_operation(s)?;
        let (s, _) = utils::extract_whitespaces(s);
        let (s, op) = Operations::new(s)?;
        let (s, _) = utils::extract_whitespaces(s);
        let (s, rhs) = Self::new_non_operation(s)?;
        Ok((s, Self::Operation {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
            op
        }))
    }

    fn new_number(s:&str) -> Result<(&str, Self), String> {
        Number::new(s).map(|(s, number)| (s, Self::Number(number)))
    }

    pub(crate) fn eval(&self, env: &Env) -> Result<Val, String> {
        match self {
            Self::Number(Number(n)) => Ok(Val::Number(*n)),
            Self::Operation { lhs, rhs, op } => {
                let lhs = lhs.eval(env)?;
                let rhs = rhs.eval(env)?;

                let (lhs, rhs) = match (lhs, rhs) {
                    (Val::Number(lhs), Val::Number(rhs)) => (lhs, rhs),
                    _ => return Err("cannot evaluate operation whose left-hand side and right-hand side are not both numbers".to_string()),
                };

                let res = match op {
                    Operations::Add => lhs + rhs,
                    Operations::Sub => lhs - rhs,
                    Operations::Mul => lhs * rhs,
                    Operations::Div => lhs / rhs,
                };

                Ok(Val::Number(res))
            },
            Self::FuncCall(func_call) => func_call.eval(env),
            Self::BindingUsage(binding_usage) => binding_usage.eval(env),
            Self::Block(block) => block.eval(env),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::statements::Statement;

    #[test]
    fn parse_number() {
        assert_eq!(Number::new("123"), Ok(("", Number(123))));
    }

    #[test]
    fn parse_number_as_expr() {
        assert_eq!(Expression::new("456"), Ok(("", Expression::Number(Number(456)))));
    }

    #[test]
    fn parse_addition() {
        assert_eq!(Operations::new("+"), Ok(("", Operations::Add)));
    }

    #[test]
    fn parse_subtraction() {
        assert_eq!(Operations::new("-"), Ok(("", Operations::Sub)));
    }

    #[test]
    fn parse_multiplication() {
        assert_eq!(Operations::new("*"), Ok(("", Operations::Mul)));
    }

    #[test]
    fn parse_division() {
        assert_eq!(Operations::new("/"), Ok(("", Operations::Div)));
    }

    #[test]
    fn add_numbers() {
        assert_eq!(
            Expression::new("1+2"),
            Ok((
                "",
                Expression::Operation {
                    lhs: Box::new(Expression::Number(Number(1))),
                    rhs: Box::new(Expression::Number(Number(2))),
                    op: Operations::Add,
                }
            )),
        );
    }

    #[test]
    fn parse_expression_with_whitespace() {
        assert_eq!(
            Expression::new("1 * 2"),
            Ok((
                "",
                Expression::Operation {
                    lhs: Box::new(Expression::Number(Number(1))),
                    rhs: Box::new(Expression::Number(Number(2))),
                    op: Operations::Mul,
                }
            )),
        );
    }

    #[test]
    fn eval_add() {
        assert_eq!(
            Expression::Operation {
                lhs: Box::new(Expression::Number(Number(10))),
                rhs: Box::new(Expression::Number(Number(5))),
                op: Operations::Add,
            }.eval(&Env::default()),
            Ok(Val::Number(15)),
        );
    }

    #[test]
    fn eval_subtract() {
        assert_eq!(
            Expression::Operation {
                lhs: Box::new(Expression::Number(Number(5))),
                rhs: Box::new(Expression::Number(Number(2))),
                op: Operations::Sub,
            }.eval(&Env::default()),
            Ok(Val::Number(3)),
        );
    }

    #[test]
    fn eval_multiply() {
        assert_eq!(
            Expression::Operation {
                lhs: Box::new(Expression::Number(Number(2))),
                rhs: Box::new(Expression::Number(Number(3))),
                op: Operations::Mul,
            }.eval(&Env::default()),
            Ok(Val::Number(6)),
        );
    }

    #[test]
    fn eval_division() {
        assert_eq!(
            Expression::Operation {
                lhs: Box::new(Expression::Number(Number(500))),
                rhs: Box::new(Expression::Number(Number(50))),
                op: Operations::Div,
            }.eval(&Env::default()),
            Ok(Val::Number(10)),
        );
    }

    #[test]
    fn parse_binding_usage() {
        assert_eq!(
            Expression::new("bar"),
            Ok((
                "",
                Expression::BindingUsage(BindingUsage {
                    name: "bar".to_string(),
                }),
            )),
        );
    }

    #[test]
    fn parse_block() {
        assert_eq!(
            Expression::new("{ 200 }"),
            Ok((
                "",
                Expression::Block(Block {
                    stmts: vec![Statement::Expression(Expression::Number(Number(200)))],
                }),
            )),
        );
    }

    #[test]
    fn eval_binding_usage() {
        let mut env = Env::default();
        env.store_binding("ten".to_string(), Val::Number(10));
        assert_eq!(
            Expression::BindingUsage(BindingUsage {
                name: "ten".to_string(),
            }).eval(&env),
            Ok(Val::Number(10)),
        );
    }

    #[test]
    fn eval_non_number_operation() {
        assert_eq!(
            Expression::Operation {
                lhs: Box::new(Expression::Number(Number(10))),
                rhs: Box::new(Expression::Block(Block { stmts: Vec::new() })),
                op: Operations::Add,
            }
            .eval(&Env::default()),
            Err("cannot evaluate operation whose left-hand side and right-hand side are not both numbers".to_string()),
        );
    }

    #[test]
    fn parse_func_call() {
        assert_eq!(
            Expression::new("add 1 2"),
            Ok((
                "",
                Expression::FuncCall(FuncCall {
                    callee: "add".to_string(),
                    params: vec![Expression::Number(Number(1)), Expression::Number(Number(2))],
                }),
            )),
        );
    }

    #[test]
    fn eval_func_call() {
        let mut env = Env::default();

        env.store_func(
            "add".to_string(),
            vec!["x".to_string(), "y".to_string()],
            Statement::Expression(Expression::Operation {
                lhs: Box::new(Expression::BindingUsage(BindingUsage {
                    name: "x".to_string(),
                })),
                rhs: Box::new(Expression::BindingUsage(BindingUsage {
                    name: "y".to_string(),
                })),
                op: Operations::Add,
            }),
        );

        assert_eq!(
            Expression::FuncCall(FuncCall {
                callee: "add".to_string(),
                params: vec![Expression::Number(Number(2)), Expression::Number(Number(2))],
            })
            .eval(&env),
            Ok(Val::Number(4)),
        );
    }
}
