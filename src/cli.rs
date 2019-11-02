use getopts::Options;

pub fn get_opts() -> Options {
    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help text");
    opts.optflag("v", "verbose", "print debug info on each instruction");
    opts
}

pub struct Config {
    pub filename: String,
    pub verbose: bool,
}

pub fn get_config() -> Result<Config, ()> {
    let env_args: Vec<String> = std::env::args().collect();
    if let Ok(mut matches) = get_opts().parse(&env_args[1..]) {
        if matches.opt_present("h") {
            Err(())
        } else {
            Ok(Config {
                filename: matches.free.pop().ok_or(())?,
                verbose: matches.opt_present("v"),
            })
        }
    } else {
        Err(())
    }
}

pub fn print_usage() {
    let program = std::env::args().next().unwrap();
    let brief = format!("Usage: {} [options] <filename>", program);
    println!(
        "Metatape v{}\n\n{}",
        env!("CARGO_PKG_VERSION"),
        get_opts().usage(&brief)
    );
}
