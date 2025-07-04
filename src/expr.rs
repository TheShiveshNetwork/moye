pub mod block;
pub mod binding_usage;

use crate::env::Env;
use crate::utils;
use crate::val::Val;
use binding_usage::BindingUsage;
use block::Block;

#[derive(Debug, PartialEq)]
pub struct Number(pub i32);

impl Number {
    pub fn new(s:&str) -> Result<(&str, Self), String> {
        let (s, number) = utils::extract_digits(s)?;
        Ok((s, Self(number.parse().unwrap())))
    }
}

#[derive(Debug, PartialEq)]
pub enum Operations {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operations {
    pub fn new(s:&str) -> Result<(&str, Self), String> {
        utils::tag("+", s)
            .map(|s| (s, Self::Add))
            .or_else(|_| utils::tag("-", s).map(|s| (s, Self::Sub)))
            .or_else(|_| utils::tag("*", s).map(|s| (s, Self::Mul)))
            .or_else(|_| utils::tag("/", s).map(|s| (s, Self::Div)))
    }
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Number(Number),
    Operation { lhs: Number, rhs: Number, op: Operations },
    BindingUsage(BindingUsage),
    Block(Block),
}

impl Expression {
    pub fn new(s:&str) -> Result<(&str, Self), String> {
        Self::new_operation(s)
            .or_else(|_| Self::new_number(s))
            .or_else(|_| {
                BindingUsage::new(s).map(|(s, binding_usage)| (s, Self::BindingUsage(binding_usage)))
            })
            .or_else(|_| Block::new(s).map(|(s, block)| (s, Self::Block(block))))
    }

    fn new_operation(s:&str) -> Result<(&str, Self), String> {
        let (s, lhs) = Number::new(s)?;
        let (s, _) = utils::extract_whitespaces(s);
        let (s, op) = Operations::new(s)?;
        let (s, _) = utils::extract_whitespaces(s);
        let (s, rhs) = Number::new(s)?;
        Ok((s, Self::Operation { lhs, rhs, op }))
    }

    fn new_number(s:&str) -> Result<(&str, Self), String> {
        Number::new(s).map(|(s, number)| (s, Self::Number(number)))
    }

    pub(crate) fn eval(&self, env: &Env) -> Result<Val, String> {
        match self {
            Self::Number(Number(n)) => Ok(Val::Number(*n)),
            Self::Operation { lhs, rhs, op } => {
                let Number(lhs) = lhs;
                let Number(rhs) = rhs;
                let res = match op {
                    Operations::Add => lhs + rhs,
                    Operations::Sub => lhs - rhs,
                    Operations::Mul => lhs * rhs,
                    Operations::Div => lhs / rhs,
                };
                Ok(Val::Number(res))
            },
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
                    lhs: Number(1),
                    rhs: Number(2),
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
                    lhs: Number(1),
                    rhs: Number(2),
                    op: Operations::Mul,
                }
            )),
        );
    }

    #[test]
    fn eval_add() {
        assert_eq!(
            Expression::Operation {
                lhs: Number(10),
                rhs: Number(5),
                op: Operations::Add,
            }.eval(&Env::default()),
            Ok(Val::Number(15)),
        );
    }

    #[test]
    fn eval_subtract() {
        assert_eq!(
            Expression::Operation {
                lhs: Number(5),
                rhs: Number(2),
                op: Operations::Sub,
            }.eval(&Env::default()),
            Ok(Val::Number(3)),
        );
    }

    #[test]
    fn eval_multiply() {
        assert_eq!(
            Expression::Operation {
                lhs: Number(2),
                rhs: Number(3),
                op: Operations::Mul,
            }.eval(&Env::default()),
            Ok(Val::Number(6)),
        );
    }

    #[test]
    fn eval_division() {
        assert_eq!(
            Expression::Operation {
                lhs: Number(500),
                rhs: Number(50),
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
}
