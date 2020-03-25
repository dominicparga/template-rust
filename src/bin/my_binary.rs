use log::{error, info};
use template_rust::helpers;

fn main() {
    // process user-input
    let matches = parse_cmdline();
    match helpers::init_logging(matches.value_of("log").unwrap(), vec![]) {
        Ok(_) => (),
        Err(msg) => {
            error!("{}", msg);
            return;
        }
    };

    info!("Hello! Execute 'cargo run -- --help' for more details.");
}

fn parse_cmdline<'a>() -> clap::ArgMatches<'a> {
    // arg: quiet
    let tmp = &[
        "Sets the logging-level by setting environment-variable 'RUST_LOG'.",
        "The env-variable 'RUST_LOG' has precedence.",
        "It takes values of modules, e.g.",
        "export RUST_LOG='warn,my_example=info'",
        "for getting warn's by default, but 'info' about the others",
    ]
    .join("\n");
    let arg_log_level = clap::Arg::with_name("log")
        .long("log")
        .short("l")
        .value_name("FILTER-LEVEL")
        .help(tmp)
        .takes_value(true)
        .required(false)
        .default_value("INFO")
        .possible_values(&vec!["TRACE", "DEBUG", "INFO", "WARN", "ERROR"]);

    // all
    clap::App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .long_about((&["", "Here could stand your help-message."].join("\n")).as_ref())
        .arg(arg_log_level)
        .get_matches()
}
