use crate::expr::Expression;
use crate::binding_def::BindingDef;
use crate::val::Val;
use crate::env::Env;
use crate::func_def::FuncDef;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Statement {
    BindingDef(BindingDef),
    Expression(Expression),
    FuncDef(FuncDef),
}

impl Statement {
    pub(crate) fn new(s:&str) -> Result<(&str, Self), String> {
        BindingDef::new(s)
            .map(|(s, binding_def)| (s, Self::BindingDef(binding_def)))
            .or_else(|_| FuncDef::new(s).map(|(s, func_def)| (s, Self::FuncDef(func_def))))
            .or_else(|_| Expression::new(s).map(|(s, expr)| (s, Self::Expression(expr))))
    }

    pub(crate) fn eval(&self, env: &mut Env) -> Result<Val, String> {
        match self {
            Self::BindingDef(binding_def) => {
                binding_def.eval(env)?;
                Ok(Val::Unit)
            },
            Self::FuncDef(func_def) => {
                func_def.eval(env)?;
                Ok(Val::Unit)
            },
            Self::Expression(expr) => expr.eval(env),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::{BindingUsage, Number, Operations};

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
                    lhs: Box::new(Expression::Number(Number(1))),
                    rhs: Box::new(Expression::Number(Number(1))),
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

    #[test]
    fn parse_function_def() {
        assert_eq!(
            Statement::new("fun name x => x"),
            Ok((
                "",
                Statement::FuncDef(FuncDef {
                    name: "name".to_string(),
                    params: vec!["x".to_string()],
                    body: Box::new(Statement::Expression(Expression::BindingUsage(BindingUsage {
                        name: "x".to_string(),
                    }))),
                }),
            )),
        );
    }

    #[test]
    fn eval_func_def() {
        assert_eq!(
            Statement::FuncDef(FuncDef {
                name: "always_return_one".to_string(),
                params: Vec::new(),
                body: Box::new(Statement::Expression(Expression::Number(Number(1)))),
            })
            .eval(&mut Env::default()),
            Ok(Val::Unit),
        );
    }
}
