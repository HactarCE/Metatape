mod debug;
mod parser;
mod program;
mod runtime;
mod tape;

pub type Program = program::Program;
pub type Runtime = runtime::Runtime;

pub fn program_from_file(filename: &str) -> Result<Program, String> {
    Ok(filename)
        .and_then(|filename| std::fs::read_to_string(filename).map_err(|err| err.to_string()))
        .and_then(|file_contents| parser::parse(file_contents).map_err(|err| err.to_string()))
}

pub fn runtime_from_file(filename: &str) -> Result<Runtime, String> {
    program_from_file(filename).map(|program| Runtime::new(program))
}
