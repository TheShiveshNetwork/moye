use crate::utils;
use crate::statements::Statement;
use crate::env::Env;
use crate::val::Val;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Block {
    pub(crate) stmts: Vec<Statement>,
}

impl Block {
    pub(super) fn new(s:&str) -> Result<(&str, Self), String> {
        let s = utils::tag("{", s)?;
        let (s, _) = utils::extract_whitespaces(s);

        let (s, stmts) = utils::sequence(Statement::new, utils::extract_whitespaces, s)?;

        let (s, _) = utils::extract_whitespaces(s);
        let s = utils::tag("}", s)?;

        Ok((s, Block {stmts}))
    }

    pub(super) fn eval(&self, env: &Env) -> Result<Val, String> {
        if self.stmts.is_empty() {
            return Ok(Val::Unit);
        }
        let mut env = env.create_child();
        let stmts_except_last = &self.stmts[..self.stmts.len() - 1];
        for stmt in stmts_except_last {
            stmt.eval(&mut env)?;
        }

        self.stmts.last().unwrap().eval(&mut env)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::{BindingUsage, Expression, Number, Operations};
    use crate::binding_def::BindingDef;

    #[test]
    fn parse_empty_block() {
        assert_eq!(Block::new("{}"), Ok(("", Block {stmts: Vec::new()} )));
    }

    #[test]
    fn parse_empty_block_with_whitespace() {
        assert_eq!(Block::new("{   }"), Ok(("", Block { stmts: Vec::new() })));
    }

    #[test]
    fn parse_block_with_one_stmt() {
        assert_eq!(
            Block::new("{ 5 }"),
            Ok((
                "",
                Block {
                    stmts: vec![Statement::Expression(Expression::Number(Number(5)))],
                },
            )),
        );
    }

    #[test]
    fn parse_block_with_multiple_statements() {
        assert_eq!(
            Block::new("{
                let a = 10
                let b = a
                b
            }"),
            Ok((
                "",
                Block {
                    stmts: vec![
                        Statement::BindingDef(BindingDef {
                            name: "a".to_string(),
                            value: Expression::Number(Number(10)),
                        }),
                        Statement::BindingDef(BindingDef {
                            name: "b".to_string(),
                            value: Expression::BindingUsage(BindingUsage {
                                name: "a".to_string(),
                            }),
                        }),
                        Statement::Expression(Expression::BindingUsage(BindingUsage {
                            name: "b".to_string(),
                        })),
                    ],
                },
            ))
        );
    }

    #[test]
    fn eval_block() {
        assert_eq!(
            Expression::Block(Block { stmts: vec![Statement::Expression(Expression::Number(Number(10)))] })
                .eval(&Env::default()),
            Ok(Val::Number(10)),
        );
    }

    #[test]
    fn eval_empty_block() {
        assert_eq!(
            Block { stmts: Vec::new() }.eval(&Env::default()),
            Ok(Val::Unit),
        );
    }

    #[test]
    fn eval_block_with_one_expr() {
        assert_eq!(
            Block {
                stmts: vec![Statement::Expression(Expression::Number(Number(25)))],
            }
            .eval(&Env::default()),
            Ok(Val::Number(25)),
        );
    }

    #[test]
    fn eval_block_with_binding_def_and_usage() {
        assert_eq!(
            Block {
                stmts: vec![
                    Statement::BindingDef(BindingDef {
                        name: "one".to_string(),
                        value: Expression::Number(Number(1)),
                    }),
                    Statement::Expression(Expression::BindingUsage(BindingUsage {
                        name: "one".to_string(),
                    })),
                ],
            }
            .eval(&Env::default()),
            Ok(Val::Number(1)),
        );
    }

    #[test]
    fn eval_block_with_multiple_binding_defs() {
        assert_eq!(
            Block {
                stmts: vec![
                    Statement::BindingDef(BindingDef {
                        name: "foo".to_string(),
                        value: Expression::Number(Number(5)),
                    }),
                    Statement::BindingDef(BindingDef {
                        name: "bar".to_string(),
                        value: Expression::Number(Number(4)),
                    }),
                    Statement::BindingDef(BindingDef {
                        name: "baz".to_string(),
                        value: Expression::Number(Number(3)),
                    }),
                ],
            }
            .eval(&Env::default()),
            Ok(Val::Unit),
        );
    }

    #[test]
    fn eval_block_with_multiple_exprs() {
        assert_eq!(
            Block {
                stmts: vec![
                    Statement::Expression(Expression::Number(Number(100))),
                    Statement::Expression(Expression::Number(Number(30))),
                    Statement::Expression(Expression::Operation {
                        lhs: Box::new(Expression::Number(Number(10))),
                        rhs: Box::new(Expression::Number(Number(7))),
                        op: Operations::Sub,
                    }),
                ],
            }
            .eval(&Env::default()),
            Ok(Val::Number(3)),
        );
    }

    #[test]
    fn eval_block_using_bindings_from_parent_env() {
        let mut env = Env::default();
        env.store_binding("foo".to_string(), Val::Number(2));

        assert_eq!(
            Block {
                stmts: vec![
                    Statement::BindingDef(BindingDef {
                        name: "baz".to_string(),
                        value: Expression::BindingUsage(BindingUsage {
                            name: "foo".to_string(),
                        }),
                    }),
                    Statement::Expression(Expression::BindingUsage(BindingUsage {
                        name: "baz".to_string(),
                    })),
                ],
            }
            .eval(&env),
            Ok(Val::Number(2)),
        );
    }
}
