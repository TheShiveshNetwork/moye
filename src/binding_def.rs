/*
 * Bindings are the immutable variable definitions
 */

use crate::expr::Expression;
use crate::utils;
use crate::env::Env;

#[derive(Debug, PartialEq)]
pub struct BindingDef {
    pub name: String,
    pub value: Expression,
}

impl BindingDef {
    pub fn new(s:&str) -> Result<(&str, Self), String> {
        let s = utils::tag("let", s)?;
        let (s, _) = utils::extract_whitespaces_with_error(s)?;
        let (s, name) = utils::extract_ident(s)?;
        let (s, _) = utils::extract_whitespaces(s);
        let s = utils::tag("=", s)?;
        let (s, _) = utils::extract_whitespaces(s);
        let (s, val) = Expression::new(s)?;
        Ok((s, Self {
            name: name.to_string(),
            value: val,
        }))
    }

    pub(crate) fn eval(&self, env: &mut Env) -> Result<(), String> {
        env.store_binding(self.name.clone(), self.value.eval(env)?);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::{Number, Operations};

    #[test]
    fn parse_binding_def() {
        assert_eq!(
            BindingDef::new("let a = 10 / 2"),
            Ok((
                "",
                BindingDef {
                    name: "a".to_string(),
                    value: Expression::Operation {
                        lhs: Number(10),
                        rhs: Number(2),
                        op: Operations::Div,
                    },
                },
            )),
        );
    }

    #[test]
    fn cannot_parse_binding_def_without_space_after_let() {
        assert_eq!(BindingDef::new("letabc=1+2"), Err("expected a space".to_string()))
    }
}
