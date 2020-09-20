mod config;
mod decoder;

use std::fs;
use std::io::{Read, Write};
use std::path::Path;

use config as cfg;

fn main() {
    let origin = fs::File::open(&cfg::get().target).unwrap();

    let mut decoder;
    match decoder::new(origin) {
        Some(val) => {
            decoder = val;
        }
        None => {
            return;
        }
    }
    let mut audio = fs::File::create(Path::new(&cfg::get().target).with_extension("mp3")).unwrap();
    let mut buf = [0; 1024];
    while let Ok(len) = decoder.read(&mut buf) {
        if len == 0 {
            break;
        }

        audio.write(&buf[..len]).unwrap();
    }

    println!("End");
}

// fn read_all(file: &Path) -> Option<Vec<u8>> {
//     let mut file = match fs::File::open(file) {
//         Ok(val) => val,
//         _ => return None,
//     };

//     let mut content = Vec::with_capacity(match file.metadata() {
//         Ok(val) => val.len() as usize,
//         _ => 2048,
//     });

//     let mut buf = vec![0; 1024];

//     while let Ok(len) = file.read(&mut buf) {
//         if len == 0 {
//             break;
//         }
//         content.extend_from_slice(&buf[0..len]);
//     }

//     Some(content)
// }

// #[test]
// fn test_read_all() {
//     let filename = "src/assets/kugou_key.xz";
//     let data = read_all(Path::new(filename)).unwrap();
//     let len = data.len();
//     println!("{}", len);
// }
