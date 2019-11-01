#[macro_use]
extern crate pest_derive;

mod cli;
mod metatape;

fn main() {
    let config = cli::get_config().unwrap_or_else(|_| {
        cli::print_usage();
        std::process::exit(0);
    });

    let cli::Config { filename, verbose } = config;

    let mut runtime = metatape::runtime_from_file(&filename).unwrap_or_else(|error_msg| {
        println!("{}", error_msg);
        std::process::exit(1);
    });

    let mut result = Ok(());

    while result.is_ok() {
        if verbose {
            result = runtime.debug_step().map(|s| println!("{}", s))
        } else {
            result = runtime.step().map(|_| ());
        }
    }
    if verbose {
        println!("Program exited because {:?}", result.unwrap_err());
    }
}
