use std::io::{stdin, stdout, Write};
use crate::Result;

pub fn prompt(msg: &str) -> Result<String> {
    print!("{msg}: ");
    stdout().flush()?;
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}
