use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "squirrelt")]
#[command(about = "lsの再開発,拡張子の種類によって色を変え，ソート順をサイズ，最終更新日，ファイル名などで選択できる")]
pub struct Config {
    #[arg(short = 'a', help = "隠しファイルも表示")]
    pub show_all: bool,

    #[arg(long = "sort-size", help = "ファイルサイズに基づいてソートし、表示")]
    pub sort_size: bool,

    #[arg(short = 't', help = "最新の更新日順に表示")]
    pub sort_time: bool,

    #[arg(long, help = "generate completion files", default_value_t = false)]
    pub completions: bool,

    #[arg(default_value = ".")]
    pub path: PathBuf,
}