use log::{error, info};
use std::convert::TryFrom;
use template_rust::defaults;
use template_rust::helpers::{err, init_logging};

fn main() {
    let args = match parse_cmdline() {
        Ok(args) => args,
        Err(msg) => {
            println!("ERROR: {}", msg);
            println!();
            panic!()
        }
    };
    let result = init_logging(&args.max_log_level, &["my_binary"]);
    if let Err(msg) = result {
        error!("{}{}", msg, "\n");
        panic!("{}", msg);
    }
    let result = run(args);
    if let Err(msg) = result {
        error!("{}{}", msg, "\n");
        panic!("{}", msg);
    }
}

fn run(_args: CmdlineArgs) -> err::Feedback {
    info!("Executing binary: my_binary");
    info!("Hello!");
    info!("Your constant from compile-time is {}", defaults::FOO);
    info!("To change this, add a suffix 'FOO=41 cargo run ...'");
    info!("Execute 'cargo run --bin my_binary -- --help' for more details.");
    Ok(())
}

fn parse_cmdline<'a>() -> err::Result<CmdlineArgs> {
    let args = clap::App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .long_about("TODO Some description of the binary's purpose.");

    let args = {
        let arg_log_level = clap::Arg::with_name(constants::ids::MAX_LOG_LEVEL)
            .long("log")
            .short("l")
            .value_name("FILTER-LEVEL")
            .help(
                "Sets the logging-level according to the env-variable 'RUST_LOG'. The env-variable \
                'RUST_LOG' has precedence. It takes values of modules, e.g. export RUST_LOG='warn,\
                osmgraphing=info' for getting warn's by default, but 'info' about the others",
            )
            .takes_value(true)
            .required(false)
            .case_insensitive(true)
            .default_value("INFO")
            .possible_values(&vec!["TRACE", "DEBUG", "INFO", "WARN", "ERROR"]);
        args.arg(arg_log_level)
    };

    CmdlineArgs::try_from(args.get_matches())
}

mod constants {
    pub mod ids {
        pub const MAX_LOG_LEVEL: &str = "max-log-level";
    }
}

struct CmdlineArgs {
    max_log_level: String,
}

impl<'a> TryFrom<clap::ArgMatches<'a>> for CmdlineArgs {
    type Error = err::Msg;

    fn try_from(matches: clap::ArgMatches<'a>) -> err::Result<CmdlineArgs> {
        let max_log_level = matches
            .value_of(constants::ids::MAX_LOG_LEVEL)
            .expect(&format!("cmdline-arg: {}", constants::ids::MAX_LOG_LEVEL));

        Ok(CmdlineArgs {
            max_log_level: String::from(max_log_level),
        })
    }
}
