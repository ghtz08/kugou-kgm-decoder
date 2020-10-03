use clap::{App, Arg, ArgMatches};
use lazy_static::lazy_static;

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Config {
    pub target: String,
    pub recursive: bool,
    pub keep_file: bool,
    // pub stdout: bool,
}

pub fn get<'a>() -> &'a Config {
    lazy_static! {
        static ref CFG: Config = (|| {
            let args = parse_args();
            let target = args.value_of("target").unwrap().to_string();
            let recursive = args.is_present("recursive");
            let keep_file = args.is_present("keep");
            // let stdout = args.is_present("stdout");

            Config {
                target,
                recursive,
                keep_file,
                // stdout,
            }
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
        .arg(
            Arg::with_name("recursive")
                .short("r")
                .long("recursive")
                .help("Processing files and directories recursively"),
        )
        .arg(
            Arg::with_name("keep")
                .short("k")
                .long("keep")
                .help("Keep original file"),
        )
        // .arg(
        //     Arg::with_name("stdout")
        //         .short("c")
        //         .long("stdout")
        //         .help("Write to standard output and don't delete input files"),
        // )
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
