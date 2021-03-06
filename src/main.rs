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
    if verbose {
        runtime.set_output_fn(Box::new(|byte| {
            println!("Output byte {:#02x}: {:#?}", byte, byte as char);
        }))
    }

    let mut result = Ok(());
    while result.is_ok() {
        if verbose {
            result = runtime.debug_step()
        } else {
            result = runtime.step().map(|_| ());
        }
        if let Err(metatape::RuntimeError::Halt) = result {
            println!("HALT");
            result = runtime.unhalt();
        }
    }
    if verbose {
        println!("Program exited because {:?}", result.unwrap_err());
    }
}
