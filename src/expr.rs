use crate::utils;
use crate::val::Val;

#[derive(Debug, PartialEq)]
pub struct Number(pub i32);

impl Number {
    pub fn new(s:&str) -> (&str, Self) {
        let (s, number) = utils::extract_digits(s);
        (s, Self(number.parse().unwrap()))
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
    pub fn new(s:&str) -> (&str, Self) {
        let (s, op) = utils::extract_operator(s);
        let op = match op {
            "+" => Self::Add,
            "-" => Self::Sub,
            "*" => Self::Mul,
            "/" => Self::Div,
            _ => unreachable!(),
        };
        (s, op)
    }
}

#[derive(Debug, PartialEq)]
pub struct Expression {
    pub lhs: Number,
    pub rhs: Number,
    pub op: Operations,
}

impl Expression {
    pub fn new(s:&str) -> (&str, Self) {
        let (s, lhs) = Number::new(s);
        let (s, _) = utils::extract_whitespaces(s);
        let (s, op) = Operations::new(s);
        let (s, _) = utils::extract_whitespaces(s);
        let (s, rhs) = Number::new(s);
        (s, Self { lhs, rhs, op })
    }

    pub(crate) fn eval(&self) -> Val {
        let Number(lhs) = self.lhs;
        let Number(rhs) = self.rhs;
        let res = match self.op {
            Operations::Add => lhs + rhs,
            Operations::Sub => lhs - rhs,
            Operations::Mul => lhs * rhs,
            Operations::Div => lhs / rhs,
        };
        Val::Number(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_number() {
        assert_eq!(Number::new("123"), ("", Number(123)));
    }

    #[test]
    fn parse_addition() {
        assert_eq!(Operations::new("+"), ("", Operations::Add));
    }

    #[test]
    fn parse_subtraction() {
        assert_eq!(Operations::new("-"), ("", Operations::Sub));
    }

    #[test]
    fn parse_multiplication() {
        assert_eq!(Operations::new("*"), ("", Operations::Mul));
    }

    #[test]
    fn parse_division() {
        assert_eq!(Operations::new("/"), ("", Operations::Div));
    }

    #[test]
    fn add_numbers() {
        assert_eq!(
            Expression::new("1+2"),
            (
                "",
                Expression {
                    lhs: Number(1),
                    rhs: Number(2),
                    op: Operations::Add,
                }
            )
        );
    }

    #[test]
    fn parse_expression_with_whitespace() {
        assert_eq!(
            Expression::new("1 * 2"),
            (
                "",
                Expression {
                    lhs: Number(1),
                    rhs: Number(2),
                    op: Operations::Mul,
                }
            )
        );
    }

    #[test]
    fn eval_add() {
        assert_eq!(
            Expression {
                lhs: Number(10),
                rhs: Number(5),
                op: Operations::Add,
            }.eval(),
            Val::Number(15),
        );
    }

    #[test]
    fn eval_subtract() {
        assert_eq!(
            Expression {
                lhs: Number(5),
                rhs: Number(2),
                op: Operations::Sub,
            }.eval(),
            Val::Number(3),
        );
    }

    #[test]
    fn eval_multiply() {
        assert_eq!(
            Expression {
                lhs: Number(2),
                rhs: Number(3),
                op: Operations::Mul,
            }.eval(),
            Val::Number(6),
        );
    }

    #[test]
    fn eval_division() {
        assert_eq!(
            Expression {
                lhs: Number(500),
                rhs: Number(50),
                op: Operations::Div,
            }.eval(),
            Val::Number(10),
        );
    }
}
