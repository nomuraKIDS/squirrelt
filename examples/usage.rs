use squirrelt::{run, Config};
use std::path::PathBuf;

fn main() {
    let config = Config {
        show_all: true,
        sort_size: false,
        sort_time: false,
        path: PathBuf::from("."),
    };

    run(config).expect("failed to list current directory");
}
