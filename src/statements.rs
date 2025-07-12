use crate::expr::Expression;
use crate::binding_def::BindingDef;
use crate::val::Val;
use crate::env::Env;

#[derive(Debug, PartialEq)]
pub(crate) enum Statement {
    BindingDef(BindingDef),
    Expression(Expression),
}

impl Statement {
    pub(crate) fn new(s:&str) -> Result<(&str, Self), String> {
        BindingDef::new(s)
            .map(|(s, binding_def)| (s, Self::BindingDef(binding_def)))
            .or_else(|_| Expression::new(s).map(|(s, expr)| (s, Self::Expression(expr))))
    }

    pub(crate) fn eval(&self, env: &mut Env) -> Result<Val, String> {
        match self {
            Self::BindingDef(binding_def) => {
                binding_def.eval(env)?;
                Ok(Val::Unit)
            },
            Self::Expression(expr) => expr.eval(env),
        }
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

    #[test]
    fn eval_binding_def() {
        assert_eq!(
            Statement::BindingDef(BindingDef {
                name: "whatever".to_string(),
                value: Expression::Number(Number(-10)),
            })
            .eval(&mut Env::default()),
            Ok(Val::Unit),
        );
    }

    #[test]
    fn eval_expr() {
        assert_eq!(
            Statement::Expression(Expression::Number(Number(5))).eval(&mut Env::default()),
            Ok(Val::Number(5)),
        );
    }
}
