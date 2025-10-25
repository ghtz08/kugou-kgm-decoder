mod config;
mod decoder;

use std::fs;
use std::io::{Read, Write};
use std::path::Path;

use config as cfg;
use infer::Infer;

fn main() {
    let cfg = cfg::get();

    let files = get_all_files(&Path::new(&cfg.target), cfg.recursive);

    println!("{} files found", files.len());
    let count = decode(&files);
    println!("Completed {}/{}", count, files.len());
}

fn decode(files: &Vec<Box<Path>>) -> usize {
    let cfg = cfg::get();

    let mut count = 0usize;
    let mut buf = [0; 16 * 1024];
    for file in files {
        let mut origin = match fs::File::open(&file) {
            Ok(val) => match decoder::new(val) {
                Some(val) => val,
                None => {
                    println!(r#"Skip: "{}""#, file.display());
                    continue;
                }
            },
            Err(_) => {
                println!(r#"Skip: "{}""#, file.display());
                continue;
            }
        };

        let mut ext = "mp3";
        let mut head_buffer = [0; 128];
        origin
            .read_exact(&mut head_buffer)
            .expect("read head error");
        {
            let info: Infer = Infer::new();
            if let Some(kind) = info.get(&head_buffer) {
                ext = match kind.mime_type() {
                    "audio/midi" => "midi",
                    "audio/opus" => "opus",
                    "audio/flac" => "flac",
                    "audio/webm" => "weba",
                    "audio/wav" => "wav",
                    "audio/ogg" => "ogg",
                    "audio/aac" => "aac",
                    _ => "mp3",
                }
            }
        }

        let audio = file.with_extension(ext);
        if audio.exists()
            && !confirm(&format!(
                r#"File "{}" already exists. Overwrite?"#,
                audio.display()
            ))
        {
            continue;
        }
        let mut audio = match fs::File::create(&audio) {
            Ok(val) => val,
            Err(err) => {
                println!(r#"Unable to create file "{}", {}"#, audio.display(), err);
                continue;
            }
        };
        audio.write_all(&head_buffer).unwrap();
        while let Ok(len) = origin.read(&mut buf) {
            if len == 0 {
                break;
            }
            audio.write(&buf[..len]).unwrap();
        }
        if !cfg.keep_file {
            if let Err(err) = fs::remove_file(file) {
                println!(
                    r#"Warning: Unable to delete file "{}", {}"#,
                    file.display(),
                    err
                );
            }
        }
        println!(r#"Ok  : "{}""#, file.display());
        count += 1;
    }
    count
}

fn get_all_files(target: &Path, recursive: bool) -> Vec<Box<Path>> {
    let mut files = Vec::new();

    let meta = match fs::symlink_metadata(target) {
        Ok(val) => val,
        Err(err) => {
            println!(r#"Invalid: "{}", {}"#, target.display(), err);
            return files;
        }
    };

    if !meta.is_dir() {
        if meta.is_file() {
            files.push(Box::from(target));
        } else {
            println!(r#"Skip: "{}""#, target.display());
        }
        return files;
    }

    let all_dir = match fs::read_dir(target) {
        Ok(val) => val,
        Err(err) => {
            println!(r#"Skip: "{}", {}"#, target.display(), err);
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
