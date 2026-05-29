use squirrelt::{run, Config};
use std::env;

fn main() {
    let config = match Config::from_args(env::args_os()) {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    };

    if let Err(err) = run(config) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}
