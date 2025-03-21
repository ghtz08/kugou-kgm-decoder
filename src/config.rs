use std::sync::LazyLock;

use clap::Parser;

pub fn get<'a>() -> &'a Config {
    static CFG: LazyLock<Config> = LazyLock::new(|| Config::parse());
    &CFG
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(clap::Parser, Default)]
#[command(version, about, after_help = after_help())]
pub struct Config {
    /// The target file or folder to be processed
    #[arg()]
    pub target: String,

    /// Processing files and directories recursively
    #[clap(short, long)]
    pub recursive: bool,

    /// Keep original file
    #[clap(short, long)]
    pub keep_file: bool,
    // Write to standard output and don't delete input files
    // #[clap(short, long)]
    // stdout: bool,
}

fn after_help() -> String {
    let author = std::env!["CARGO_PKG_AUTHORS"];
    let repository = std::env!["CARGO_PKG_REPOSITORY"];

    format!("author    : {author}\nrepository: {repository}")
}
