use crate::utils;
use crate::statements::Statement;

#[derive(Debug, PartialEq)]
pub struct Block {
    pub stmts: Vec<Statement>,
}

impl Block {
    pub fn new(s:&str) -> Result<(&str, Self), String> {
        let s = utils::tag("{", s)?;
        let (s, _) = utils::extract_whitespaces(s);

        let mut s = dbg!(s);
        let mut stmts = Vec::new();

        while let Ok((new_s, stmt)) = Statement::new(s) {
            s = dbg!(new_s);
            stmts.push(stmt);
            let (new_s, _) = utils::extract_whitespaces(s);
            s = new_s;
            dbg!(s);
        }

        let (s, _) = utils::extract_whitespaces(s);
        let s = utils::tag("}", s)?;

        Ok((s, Block {stmts}))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::{BindingUsage, Block, Expression, Number};
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
}
