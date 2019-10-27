#[macro_use]
extern crate pest_derive;

mod cli;
mod metatape;

fn main() -> Result<(), String> {
    let runtime_config = cli::get_runtime_from_cli().unwrap_or_else(|err| {
        match err {
            Some(s) => println!("{}", s),
            None => cli::print_usage(),
        }
        std::process::exit(1);
    });
    let program = metatape::compile(&runtime_config.program_source)?;
    let mut runtime = metatape::make_runtime(&program, runtime_config.verbose);
    while let Ok(()) = runtime.step() {}
    Ok(())
}
