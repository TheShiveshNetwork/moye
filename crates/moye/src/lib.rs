mod val;
mod expr;
mod binding_def;
mod statements;
mod func_def;

mod env;
mod utils;

pub use env::Env;
pub use val::Val;

#[derive(Debug)]
pub struct Parse(statements::Statement);

impl Parse {
    pub fn eval(&self, env: &mut Env) -> Result<Val, String> {
        self.0.eval(env)
    }
}

pub fn parse(s:&str) -> Result<Parse, String> {
    let (s, stmt) = statements::Statement::new(s)?;
    if s.is_empty() {
        Ok(Parse(stmt))
    } else {
        Err("input was not consumed fully by parser".to_string())
    }
}

