mod parser;
mod program;
mod runtime;
mod tape;

pub type Runtime<'a> = runtime::Runtime<'a>;
pub type Program<'a> = program::Program<'a>;

pub fn compile<'a>(program_source: &'a str) -> Result<Program<'a>, String> {
    parser::parse(program_source).or_else(|parse_error| Err(parse_error.to_string()))
}

pub fn make_runtime<'a>(program: &'a Program, verbose: bool) -> Runtime<'a> {
    Runtime::new(program, verbose)
}
