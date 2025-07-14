use crate::utils;
use super::{Expression, Val, Env};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct FuncCall {
    pub(crate) callee: String,
    pub(crate) params: Vec<Expression>,
}

impl FuncCall {
    pub(super) fn new(s:&str) -> Result<(&str, Self), String> {
        let (s, callee) = utils::extract_ident(s)?;
        let (s, _) = utils::take_while(|c| c == ' ', s);
        let (s, params) = utils::non_empty_sequence(Expression::new, |s| utils::take_while(|c| c == ' ', s), s)?;

        Ok((
            s,
            Self {
                callee: callee.to_string(),
                params,
            },
        ))
    }

    pub(super) fn eval(&self, env: &Env) -> Result<Val, String> {
        let mut child_env = env.create_child();
        let (param_names, body) = env.get_func(&self.callee)?;

        let num_expected_params = param_names.len();
        let num_actual_params = self.params.len();

        if num_expected_params != num_actual_params {
            return Err(format!(
                "expected {} parameters, got {}",
                num_expected_params, num_actual_params,
            ));
        }

        for (param_name, param_expr) in param_names.into_iter().zip(&self.params) {
            let param_val = param_expr.eval(&child_env)?;
            child_env.store_binding(param_name, param_val);
        }

        body.eval(&mut child_env)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::{BindingUsage, Number, Operations};
    use crate::statements::Statement;

    #[test]
    fn parse_func_call_with_one_parameter() {
        assert_eq!(
            FuncCall::new("factorial 10"),
            Ok((
                "",
                FuncCall {
                    callee: "factorial".to_string(),
                    params: vec![Expression::Number(Number(10))],
                },
            )),
        );
    }

    #[test]
    fn eval_func_call() {
        let mut env = Env::default();
        env.store_func(
            "id".to_string(),
            vec!["x".to_string()],
            Statement::Expression(Expression::BindingUsage(BindingUsage {
                name: "x".to_string(),
            })),
        );
        assert_eq!(
            FuncCall {
                callee: "id".to_string(),
                params: vec![Expression::Number(Number(10))],
            }
            .eval(&env),
            Ok(Val::Number(10)),
        );
    }

    #[test]
    fn eval_non_existent_func_call() {
        let env = Env::default();
        assert_eq!(
            FuncCall {
                callee: "i_dont_exist".to_string(),
                params: vec![Expression::Number(Number(1))],
            }
            .eval(&env),
            Err("function with name ‘i_dont_exist’ does not exist".to_string()),
        );
    }

    #[test]
    fn eval_func_call_with_too_few_parameters() {
        let mut env = Env::default();
        env.store_func(
            "mul".to_string(),
            vec!["a".to_string(), "b".to_string()],
            Statement::Expression(Expression::Operation {
                lhs: Box::new(Expression::BindingUsage(BindingUsage {
                    name: "a".to_string(),
                })),
                rhs: Box::new(Expression::BindingUsage(BindingUsage {
                    name: "b".to_string(),
                })),
                op: Operations::Mul,
            }),
        );
        assert_eq!(
            FuncCall {
                callee: "mul".to_string(),
                params: vec![Expression::Number(Number(100))],
            }
            .eval(&env),
            Err("expected 2 parameters, got 1".to_string()),
        );
    }

    #[test]
    fn eval_func_call_with_too_many_parameters() {
        let mut env = Env::default();
        env.store_func(
            "square".to_string(),
            vec!["n".to_string()],
            Statement::Expression(Expression::Operation {
                lhs: Box::new(Expression::BindingUsage(BindingUsage {
                    name: "n".to_string(),
                })),
                rhs: Box::new(Expression::BindingUsage(BindingUsage {
                    name: "n".to_string(),
                })),
                op: Operations::Mul,
            }),
        );
        assert_eq!(
            FuncCall {
                callee: "square".to_string(),
                params: vec![Expression::Number(Number(5)), Expression::Number(Number(42))],
            }
            .eval(&env),
            Err("expected 1 parameters, got 2".to_string()),
        );
    }
}
