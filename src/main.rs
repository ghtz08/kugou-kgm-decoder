mod config;
mod decoder;

use std::fs;
use std::io::{Read, Write};
use std::path::Path;

use config as cfg;

fn main() {
    let cfg = cfg::get();

    let files = get_all_files(&Path::new(&cfg.target), cfg.recursive);

    println!("{} files found", files.len());

    let mut buf = [0; 16 * 1024];
    for file in &files {
        let mut origin = match fs::File::open(&file) {
            Ok(val) => match decoder::new(val) {
                Some(val) => val,
                None => {
                    println!("Skip: {:?}", file);
                    continue;
                }
            },
            Err(_) => {
                println!("Skip: {:?}", file);
                continue;
            }
        };
        let audio = file.with_extension("mp3");
        if audio.exists() && !confirm(&format!("File {:?} already exists. Overwrite?", audio)) {
            continue;
        }
        let mut audio = match fs::File::create(&audio) {
            Ok(val) => val,
            Err(err) => {
                println!("Unable to create file {:?}, {}", audio, err);
                continue;
            }
        };
        while let Ok(len) = origin.read(&mut buf) {
            if len == 0 {
                break;
            }
            audio.write(&buf[..len]).unwrap();
        }
        if !cfg.keep_file {
            if let Err(err) = fs::remove_file(file) {
                println!("Warning: Unable to delete file {:?}, {}", file, err);
            }
        }
        println!("Ok  : {:?}", file);
    }
}

fn get_all_files(target: &Path, recursive: bool) -> Vec<Box<Path>> {
    let mut files = Vec::new();

    let meta = match fs::symlink_metadata(target) {
        Ok(val) => val,
        Err(err) => {
            println!("Invalid: \"{:?}\", {}", target, err);
            return files;
        }
    };

    if !meta.is_dir() {
        if meta.is_file() {
            files.push(Box::from(target));
        } else {
            println!("Skip: {:?}", target);
        }
        return files;
    }

    let all_dir = match fs::read_dir(target) {
        Ok(val) => val,
        Err(err) => {
            println!("Skip: \"{:?}\", {}", target, err);
            return files;
        }
    };

    for entry in all_dir {
        let entry = match entry {
            Ok(val) => val,
            Err(err) => {
                println!("Wraning: skip an unknown file({})", err);
                continue;
            }
        };

        let meta = match entry.metadata() {
            Ok(val) => val,
            Err(err) => {
                println!("Skip: \"{:?}\", {}", entry, err);
                continue;
            }
        };
        if meta.is_dir() {
            if recursive {
                files.append(&mut get_all_files(&entry.path(), recursive));
            }
            continue;
        }

        if meta.is_file() {
            files.push(Box::from(entry.path()));
        }
    }

    files
}

fn confirm(tips: &str) -> bool {
    print!("{} (y/n): ", tips);
    std::io::stdout().flush().unwrap();
    let mut buf = [0u8; 12];

    let len = std::io::stdin().read(&mut buf).unwrap();
    if len == 1 {
        return true;
    }
    if buf[len - 1] != '\n' as u8 {
        while let Ok(len) = std::io::stdin().read(&mut buf[4..]) {
            if buf[4 + len - 1] == '\n' as u8 {
                break;
            }
        }
        return false;
    }

    len == 2 && (buf[0] == 'y' as u8 || buf[0] == 'Y' as u8)
}
