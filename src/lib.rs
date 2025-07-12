mod val;
mod expr;
mod binding_def;
mod statements;

mod env;
mod utils;


pub struct Parse(statements::Statement);

pub fn parse(s:&str) -> Result<Parse, String> {
    let (s, stmt) = statements::Statement::new(s)?;
    if s.is_empty() {
        Ok(Parse(stmt))
    } else {
        Err("input was not consumed fully by parser".to_string())
    }
}

