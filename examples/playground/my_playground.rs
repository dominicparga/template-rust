use log::{error, info};
use template_rust::helpers::{err, init_logging};

fn main() {
    init_logging("INFO", &["my_playground"]).expect("LogLevel 'INFO' does exist.");
    let result = run();
    if let Err(msg) = result {
        error!("{}", msg);
        panic!("{}", msg);
    }
}

fn run() -> err::Feedback {
    info!("Executing example: my_playground");
    info!("Hello! Execute 'cargo run -- --help' for more details.");
    Ok(())
}
