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

        let (s, stmts) = if let Ok((s, stmt)) = Statement::new(s) {
            (s, vec![stmt])
        } else {
            (s, Vec::new())
        };

        let (s, _) = utils::extract_whitespaces(s);
        let s = utils::tag("}", s)?;
        Ok((s, Block {stmts}))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::{Expression, Number};

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
}
