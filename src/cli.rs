use getopts::Options;

pub fn get_opts() -> Options {
    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help text");
    opts.optflag("v", "verbose", "print debug info on each instruction");
    opts
}

pub struct RuntimeConfig {
    pub program_source: String,
    pub verbose: bool,
}

pub fn get_runtime_from_cli<'a>() -> Result<RuntimeConfig, Option<String>> {
    let env_args: Vec<String> = std::env::args().collect();
    if let Ok(mut matches) = get_opts().parse(&env_args[1..]) {
        if matches.opt_present("h") {
            Err(None)
        } else {
            let filename = matches.free.pop().ok_or(None)?;
            let file_contents =
                std::fs::read_to_string(filename).or_else(|err| Err(err.to_string()))?;
            Ok(RuntimeConfig {
                program_source: file_contents,
                verbose: matches.opt_present("v"),
            })
        }
    } else {
        Err(None)
    }
}

pub fn print_usage() {
    let program = std::env::args().next().unwrap();
    let brief = format!("Usage: {} [options] <filename>", program);
    println!("{}", get_opts().usage(&brief));
}
