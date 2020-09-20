use clap::{App, Arg, ArgMatches};
use lazy_static::lazy_static;

pub struct Config {
    pub target: String,
}

pub fn get<'a>() -> &'a Config {
    lazy_static! {
        static ref CFG: Config = (|| {
            let args = parse_args();
            let target = args.value_of("target").unwrap().to_string();
            Config { target }
        })();
    }

    &CFG
}

fn parse_args<'a>() -> ArgMatches<'a> {
    let bin_name = std::env!["CARGO_PKG_NAME"];
    let version = std::env!["CARGO_PKG_VERSION"];
    let author = std::env!["CARGO_PKG_AUTHORS"];
    let repository = std::env!["CARGO_PKG_REPOSITORY"];

    App::new(bin_name)
        .version(version)
        .arg(
            Arg::with_name("target")
                .help("The target file or folder to be processed")
                .required(true)
                .index(1),
        )
        // .arg(Arg::with_name("recursive").short("r").long("recursive").help("Processing files and directories recursively"))
        // .arg(Arg::with_name("keep").short("k").long("keep").help("Keep original file"))
        .after_help(
            format!(
                "\
            author    : {}\n\
            repository: {}",
                author, repository
            )
            .as_str(),
        )
        .get_matches()
}
