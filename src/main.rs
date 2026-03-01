#![deny(clippy::unwrap_used)]
mod config;
mod decoder;

use std::fs;
use std::io::{self, Read, Write};
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
    files.iter().filter(|file| decode_file(file)).count()
}

fn decode_file(file: &Path) -> bool {
    let cfg = cfg::get();

    let mut origin = match fs::File::open(file).ok().and_then(decoder::new) {
        Some(val) => val,
        None => {
            println!(r#"Skip: "{}", no decoder"#, file.display());
            return false;
        }
    };

    let mut head_buffer = vec![0u8; 8192];
    let n = match origin.read(&mut head_buffer) {
        Ok(0) | Err(_) => {
            println!(r#"Skip: "{}", failed to read file head"#, file.display());
            return false;
        }
        Ok(n) => n,
    };
    head_buffer.truncate(n);

    let ext = infer_extension(file, &head_buffer, &cfg.output_extension);
    let out_path = build_out_path(file, ext);

    if out_path.exists()
        && !confirm(&format!(
            r#"File "{}" already exists. Overwrite?"#,
            out_path.display()
        ))
    {
        return false;
    }

    let mut audio = match fs::File::create(&out_path) {
        Ok(val) => val,
        Err(err) => {
            println!(r#"Unable to create file "{}", {}"#, out_path.display(), err);
            return false;
        }
    };

    if let Err(err) = write_decoded(&mut audio, &head_buffer, &mut origin) {
        println!(
            r#"Write error: "{}" -> "{}", {}"#,
            file.display(),
            out_path.display(),
            err
        );
        let _ = fs::remove_file(&out_path);
        return false;
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
    true
}

fn infer_extension<'a>(file: &'a Path, head: &[u8], override_ext: &'a str) -> &'a str {
    if !override_ext.is_empty() {
        return override_ext;
    }
    if let Some(kind) = Infer::new().get(head) {
        return kind.extension();
    }
    file.extension().and_then(|e| e.to_str()).unwrap_or("mp3")
}

fn build_out_path(file: &Path, ext: &str) -> std::path::PathBuf {
    let stem = file
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or_else(|| {
            file.file_name()
                .and_then(|f| f.to_str())
                .unwrap_or("output")
        });

    // Strip a double extension like "song.kgm" → stem is "song.kgm", output should be "song"
    const KGM_EXT: &str = ".kgm";
    let stem = match stem.len().checked_sub(KGM_EXT.len()) {
        Some(pos) if stem[pos..].eq_ignore_ascii_case(KGM_EXT) => &stem[..pos],
        _ => stem,
    };

    file.with_file_name(format!("{}.{}", stem, ext))
}

fn write_decoded(out: &mut dyn Write, head: &[u8], rest: &mut dyn Read) -> io::Result<()> {
    out.write_all(head)?;
    io::copy(rest, out)?;
    Ok(())
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
