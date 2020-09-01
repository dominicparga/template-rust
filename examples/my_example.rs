use log::{error, info};
use template_rust::helpers::{err, init_logging};

fn main() {
    init_logging("INFO", &["my_example"]).expect("LogLevel 'INFO' does exist.");
    let result = run();
    if let Err(msg) = result {
        error!("{}{}", msg, "\n");
        panic!("{}", msg);
    }
}

fn run() -> err::Feedback {
    info!("Executing example: my_example");
    info!("Hello! Execute 'cargo run -- --help' for more details.");
    Ok(())
}
