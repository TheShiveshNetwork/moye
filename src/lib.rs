mod utils;

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
        let (s, op) = Operations::new(s);
        let (s, rhs) = Number::new(s);
        (s, Self { lhs, rhs, op })
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
                    op: Operations::Add
                }
            )
        );
    }
}
