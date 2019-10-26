mod lexical;
pub mod tokens;

use super::program::Program;

pub fn parse(string: &str) -> Result<Program, String> {
    lexical::tokenize(string).or_else(|err| Err(err.to_string()))
}
