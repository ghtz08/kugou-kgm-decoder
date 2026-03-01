#![deny(clippy::unwrap_used)]
mod config;
mod decoder;

use std::fs;
use std::io::{Read, Write};
use std::path::Path;

use config as cfg;
use infer::Infer;

fn main() {
    let cfg = cfg::get();

    let files = get_all_files(Path::new(&cfg.target), cfg.recursive);

    println!("{} files found", files.len());
    let count = decode(&files);
    println!("Completed {}/{}", count, files.len());
}

fn decode(files: &[Box<Path>]) -> usize {
    let cfg = cfg::get();

    let mut count = 0usize;
    let mut buf = [0; 16 * 1024];
    'file_loop: for file in files {
        let mut origin = match fs::File::open(file) {
            Ok(val) => match decoder::new(val) {
                Some(val) => val,
                None => {
                    println!(r#"Skip: "{}", no decoder"#, file.display());
                    continue;
                }
            },
            Err(_) => {
                println!(r#"Skip: "{}", file not found"#, file.display());
                continue;
            }
        };

        let mut head_buffer = vec![0u8; 8192];
        let n = match origin.read(&mut head_buffer) {
            Ok(n) => n,
            Err(err) => {
                println!(r#"Skip: "{}", read head error: {err}"#, file.display());
                continue;
            }
        };

        if n == 0 {
            println!(r#"Skip: "{}", file is empty"#, file.display());
            continue;
        }

        head_buffer.truncate(n);

        let ext = if !cfg.output_extension.is_empty() {
            cfg.output_extension.as_str()
        } else {
            let mut ext = file.extension().and_then(|e| e.to_str()).unwrap_or("mp3");

            if let Some(kind) = Infer::new().get(&head_buffer) {
                ext = kind.extension();
            }
            ext
        };

        let stem_os = file
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or_else(|| {
                file.file_name()
                    .and_then(|f| f.to_str())
                    .unwrap_or("output")
            });
        let stem = if stem_os.to_lowercase().ends_with(".kgm") {
            let len = stem_os.len();
            stem_os[..len - 4].to_string()
        } else {
            stem_os.to_string()
        };

        let out_path = file.with_file_name(format!("{}.{}", stem, ext));

        if out_path.exists()
            && !confirm(&format!(
                r#"File "{}" already exists. Overwrite?"#,
                out_path.display()
            ))
        {
            continue;
        }

        let mut audio = match fs::File::create(&out_path) {
            Ok(val) => val,
            Err(err) => {
                println!(r#"Unable to create file "{}", {}"#, out_path.display(), err);
                continue;
            }
        };

        if let Err(err) = audio.write_all(&head_buffer) {
            println!(
                r#"Write head error: "{}" -> "{}", {}"#,
                file.display(),
                out_path.display(),
                err
            );
            let _ = fs::remove_file(&out_path);
            continue;
        }

        loop {
            match origin.read(&mut buf) {
                Ok(0) => break,
                Ok(len) => {
                    if let Err(err) = audio.write_all(&buf[..len]) {
                        println!(
                            r#"Write error: "{}" -> "{}", {}"#,
                            file.display(),
                            out_path.display(),
                            err
                        );
                        let _ = fs::remove_file(&out_path);
                        continue 'file_loop;
                    }
                }
                Err(err) => {
                    println!(
                        r#"Read error while writing: "{}" -> "{}", {}"#,
                        file.display(),
                        out_path.display(),
                        err
                    );
                    let _ = fs::remove_file(&out_path);
                    continue 'file_loop;
                }
            }
        }

        if !cfg.keep_file
            && let Err(err) = fs::remove_file(file)
        {
            println!(
                r#"Warning: Unable to delete file "{}", {}"#,
                file.display(),
                err
            );
        }

        println!(r#"Ok  : "{}" -> "{}""#, file.display(), out_path.display());
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
            println!(
                r#"Skip: "{}", file is not a regular file"#,
                target.display()
            );
        }
        return files;
    }

    let all_dir = match fs::read_dir(target) {
        Ok(val) => val,
        Err(err) => {
            println!(
                r#"Skip: "{}", failed to read directory: {}"#,
                target.display(),
                err
            );
            return files;
        }
    };

    for entry in all_dir {
        let entry = match entry {
            Ok(val) => val,
            Err(err) => {
                println!("Warning: skip an unknown file({})", err);
                continue;
            }
        };

        let meta = match entry.metadata() {
            Ok(val) => val,
            Err(err) => {
                println!("Skip: failed to read entry metadata: {}", err);
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
    loop {
        print!("{} (y/n): ", tips);
        std::io::stdout().flush().expect("flush stdout failed");
        let mut input = String::new();
        if std::io::stdin().read_line(&mut input).is_err() {
            return false;
        }
        let input = input.trim().to_lowercase();

        if input == "y" || input == "yes" {
            return true;
        } else if input == "n" || input == "no" {
            return false;
        } else {
            println!("Invalid input, please enter 'y' or 'n'.");
        }
    }
}
