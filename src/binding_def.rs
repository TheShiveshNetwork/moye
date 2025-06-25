/*
 * Bindings are the immutable variable definitions
 */

use crate::expr::Expression;
use crate::utils;
use crate::env::Env;

#[derive(Debug, PartialEq)]
pub struct BindingDef {
    name: String,
    value: Expression,
}

impl BindingDef {
    pub fn new(s:&str) -> (&str, Self) {
        let s = utils::tag("let", s);
        let (s, _) = utils::extract_whitespaces(s);
        let (s, name) = utils::extract_ident(s);
        let (s, _) = utils::extract_whitespaces(s);
        let s = utils::tag("=", s);
        let (s, _) = utils::extract_whitespaces(s);
        let (s, val) = Expression::new(s);
        (s, Self {
            name: name.to_string(),
            value: val,
        })
    }

    pub(crate) fn eval(&self, env: &mut Env) {
        env.store_binding(self.name.clone(), self.value.eval());
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
            (
                "",
                BindingDef {
                    name: "a".to_string(),
                    value: Expression {
                        lhs: Number(10),
                        rhs: Number(2),
                        op: Operations::Div,
                    },
                },
            ),
        );
    }
}
