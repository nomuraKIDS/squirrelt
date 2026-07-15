use squirrelt::{run, Config};
use clap::Parser; // Config::parse() を使うために必要

mod gencomp; 

fn main() {
    // Config::from_args(...) の代わりに、clapの機能で引数をパースします
    let config = Config::parse();

    if config.completions {
        gencomp::generate(std::path::Path::new("completions"));
        println!("補完ファイルを completions/ フォルダに出力しました。");
        return;
    }

    if let Err(err) = run(config) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}