use crate::utils;
use crate::env::Env;
use crate::statements::Statement;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct FuncDef {
    pub(crate) name: String,
    pub(crate) params: Vec<String>,
    pub(crate) body: Box<Statement>,
}

impl FuncDef {
    pub(crate) fn new(s: &str) -> Result<(&str, Self), String> {
        let s = utils::tag("fun", s)?;
        let (s, _) = utils::extract_whitespaces_with_error(s)?;
        let (s, name) = utils::extract_ident(s)?;
        dbg!(s);
        dbg!(name);
        let (s, _) = utils::extract_whitespaces(s);

        let (s, params) = utils::sequence(
            |s| utils::extract_ident(s).map(|(s, ident)| (s, ident.to_string())),
            utils::extract_whitespaces,
            s,
        )?;

        dbg!(s);
        let s = utils::tag("=>", s)?;
        let (s, _) = utils::extract_whitespaces(s);
        let (s, body) = Statement::new(s)?;
        Ok((
            s,
            Self {
                name: name.to_string(),
                params,
                body: Box::new(body),
            },
        ))
    }

    pub(crate) fn eval(&self, env: &mut Env) -> Result<(), String> {
        env.store_func(self.name.clone(), self.params.clone(), *self.body.clone());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::{BindingUsage, Operations, Block, Expression};

    #[test]
    fn parse_func_def_with_no_params_and_empty_body() {
        assert_eq!(
            FuncDef::new("fun nothing => {}"),
            Ok((
                "",
                FuncDef {
                    name: "nothing".to_string(),
                    params: Vec::new(),
                    body: Box::new(Statement::Expression(Expression::Block(Block { stmts: Vec::new() }))),
                }
            )),
        );
    }

    #[test]
    fn parse_func_def_with_one_param_and_empty_body() {
        assert_eq!(
            FuncDef::new("fun greet name => {}"),
            Ok((
                "",
                FuncDef {
                    name: "greet".to_string(),
                    params: vec!["name".to_string()],
                    body: Box::new(Statement::Expression(Expression::Block(Block { stmts: Vec::new() }))),
                },
            )),
        );
    }

    #[test]
    fn parse_func_def_with_multiple_params_and_non_empty_body() {
        assert_eq!(
            FuncDef::new("fun add x y => x + y"),
            Ok((
                "",
                FuncDef {
                    name: "add".to_string(),
                    params: vec!["x".to_string(), "y".to_string()],
                    body: Box::new(Statement::Expression(Expression::Operation {
                        lhs: Box::new(Expression::BindingUsage(BindingUsage {
                            name: "x".to_string()
                        })),
                        rhs: Box::new(Expression::BindingUsage(BindingUsage {
                            name: "y".to_string()
                        })),
                        op: Operations::Add
                    }))
                }
            ))
        );
    }
}
