use crate::expr::Expression;
use crate::binding_def::BindingDef;

#[derive(Debug, PartialEq)]
pub enum Statement {
    BindingDef(BindingDef),
    Expression(Expression),
}

impl Statement {
    pub fn new(s:&str) -> Result<(&str, Self), String> {
        BindingDef::new(s)
            .map(|(s, binding_def)| (s, Self::BindingDef(binding_def)))
            .or_else(|_| Expression::new(s).map(|(s, expr)| (s, Self::Expression(expr))))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::{Number, Operations};

    #[test]
    fn parse_binding_def() {
        assert_eq!(
            Statement::new("let a = 10"), 
            Ok((
                "",
                Statement::BindingDef(BindingDef {
                    name: "a".to_string(),
                    value: Expression::Number(Number(10)),
                }),
            )),
        );
    }

    #[test]
    fn parse_expression() {
        assert_eq!(
            Statement::new("1+1"),
            Ok((
                "",
                Statement::Expression(Expression::Operation {
                    lhs: Number(1),
                    rhs: Number(1),
                    op: Operations::Add,
                }),
            ))
        );
    }
}
