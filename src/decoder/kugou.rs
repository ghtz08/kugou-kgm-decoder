use lazy_static::lazy_static;
use std::io::Read;
use std::ops::Range;
use xz2::read::XzDecoder;

use super::Decoder;

pub struct KuGou<'a> {
    origin: Box<dyn Read + 'a>,
    own_key: [u8; KuGou::OWN_KEY_LEN as usize],
    pos: u64,
}

impl<'a> KuGou<'a> {
    const HEADER_LEN: u64 = 1024;
    const OWN_KEY_LEN: u64 = 17;
    const PUB_KEY_LEN: u64 = 1170494464;
    const PUB_KEY_LEN_MAGNIFICATION: u64 = 16;
    const MAGIC_HEADER: [u8; 28] = [
        0x7c, 0xd5, 0x32, 0xeb, 0x86, 0x02, 0x7f, 0x4b, 0xa8, 0xaf, 0xa6, 0x8e, 0x0f, 0xff, 0x99,
        0x14, 0x00, 0x04, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
    ];

    fn get_pub_key(index: Range<u64>) -> &'static [u8] {
        // TODO: 对 key 进行惰性解码（需要解决随之带来的静态变量线程安全问题）
        static KGM_KEY_XZ: &[u8] = include_bytes!("../assets/kugou_key.xz");
        lazy_static! {
            static ref KEYS: Vec<u8> = (|| {
                let mut xz_decoder = XzDecoder::new(Bytes::new(KGM_KEY_XZ));
                let mut key =
                    vec![0; (KuGou::PUB_KEY_LEN / KuGou::PUB_KEY_LEN_MAGNIFICATION) as usize];
                match xz_decoder.read_exact(&mut key) {
                    Ok(_) => key,
                    _ => {
                        panic!("Failed to decode the KuGou key")
                    }
                }
            })();
        }

        &KEYS[(index.start / KuGou::PUB_KEY_LEN_MAGNIFICATION) as usize
            ..(index.end / KuGou::PUB_KEY_LEN_MAGNIFICATION + 1) as usize]
    }
}

impl<'a> Decoder<'a> for KuGou<'a> {
    fn new(origin: impl Read + 'a) -> Self {
        match KuGou::try_new(origin) {
            Some(val) => val,
            None => panic!("Invalid KGM data"),
        }
    }

    fn decodeable_length_interval() -> (u64, u64) {
        (KuGou::HEADER_LEN, KuGou::HEADER_LEN + KuGou::PUB_KEY_LEN)
    }

    fn try_new(mut origin: impl Read + 'a) -> Option<Self> {
        let mut buf = [0; KuGou::HEADER_LEN as usize];
        match origin.read(&mut buf) {
            Ok(len) if len == buf.len() && buf.starts_with(&KuGou::MAGIC_HEADER) => {
                let mut own_key = [0; KuGou::OWN_KEY_LEN as usize];
                own_key[..16].copy_from_slice(&buf[0x1c..0x2c]);
                Some(KuGou {
                    origin: Box::new(origin),
                    own_key,
                    pos: 0,
                })
            }
            _ => {
                return None;
            }
        }
    }
}

impl<'a> Read for KuGou<'a> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        const PUB_KEY_MEND: [u8; 272] = [
            0xB8, 0xD5, 0x3D, 0xB2, 0xE9, 0xAF, 0x78, 0x8C, 0x83, 0x33, 0x71, 0x51, 0x76, 0xA0,
            0xCD, 0x37, 0x2F, 0x3E, 0x35, 0x8D, 0xA9, 0xBE, 0x98, 0xB7, 0xE7, 0x8C, 0x22, 0xCE,
            0x5A, 0x61, 0xDF, 0x68, 0x69, 0x89, 0xFE, 0xA5, 0xB6, 0xDE, 0xA9, 0x77, 0xFC, 0xC8,
            0xBD, 0xBD, 0xE5, 0x6D, 0x3E, 0x5A, 0x36, 0xEF, 0x69, 0x4E, 0xBE, 0xE1, 0xE9, 0x66,
            0x1C, 0xF3, 0xD9, 0x02, 0xB6, 0xF2, 0x12, 0x9B, 0x44, 0xD0, 0x6F, 0xB9, 0x35, 0x89,
            0xB6, 0x46, 0x6D, 0x73, 0x82, 0x06, 0x69, 0xC1, 0xED, 0xD7, 0x85, 0xC2, 0x30, 0xDF,
            0xA2, 0x62, 0xBE, 0x79, 0x2D, 0x62, 0x62, 0x3D, 0x0D, 0x7E, 0xBE, 0x48, 0x89, 0x23,
            0x02, 0xA0, 0xE4, 0xD5, 0x75, 0x51, 0x32, 0x02, 0x53, 0xFD, 0x16, 0x3A, 0x21, 0x3B,
            0x16, 0x0F, 0xC3, 0xB2, 0xBB, 0xB3, 0xE2, 0xBA, 0x3A, 0x3D, 0x13, 0xEC, 0xF6, 0x01,
            0x45, 0x84, 0xA5, 0x70, 0x0F, 0x93, 0x49, 0x0C, 0x64, 0xCD, 0x31, 0xD5, 0xCC, 0x4C,
            0x07, 0x01, 0x9E, 0x00, 0x1A, 0x23, 0x90, 0xBF, 0x88, 0x1E, 0x3B, 0xAB, 0xA6, 0x3E,
            0xC4, 0x73, 0x47, 0x10, 0x7E, 0x3B, 0x5E, 0xBC, 0xE3, 0x00, 0x84, 0xFF, 0x09, 0xD4,
            0xE0, 0x89, 0x0F, 0x5B, 0x58, 0x70, 0x4F, 0xFB, 0x65, 0xD8, 0x5C, 0x53, 0x1B, 0xD3,
            0xC8, 0xC6, 0xBF, 0xEF, 0x98, 0xB0, 0x50, 0x4F, 0x0F, 0xEA, 0xE5, 0x83, 0x58, 0x8C,
            0x28, 0x2C, 0x84, 0x67, 0xCD, 0xD0, 0x9E, 0x47, 0xDB, 0x27, 0x50, 0xCA, 0xF4, 0x63,
            0x63, 0xE8, 0x97, 0x7F, 0x1B, 0x4B, 0x0C, 0xC2, 0xC1, 0x21, 0x4C, 0xCC, 0x58, 0xF5,
            0x94, 0x52, 0xA3, 0xF3, 0xD3, 0xE0, 0x68, 0xF4, 0x00, 0x23, 0xF3, 0x5E, 0x0A, 0x7B,
            0x93, 0xDD, 0xAB, 0x12, 0xB2, 0x13, 0xE8, 0x84, 0xD7, 0xA7, 0x9F, 0x0F, 0x32, 0x4C,
            0x55, 0x1D, 0x04, 0x36, 0x52, 0xDC, 0x03, 0xF3, 0xF9, 0x4E, 0x42, 0xE9, 0x3D, 0x61,
            0xEF, 0x7C, 0xB6, 0xB3, 0x93, 0x50,
        ];

        // for (let i = 0; i < dataLen; i++) {
        //     let med8 = key1[i % 17] ^ audioData[i]
        //     med8 ^= (med8 & 0xf) << 4
        //     let msk8 = GetMask(i)
        //     msk8 ^= (msk8 & 0xf) << 4
        //     audioData[i] = med8 ^ msk8
        // }
        // if (raw_ext === "vpr") {
        //     for (let i = 0; i < dataLen; i++) audioData[i] ^= VprMaskDiff[i % 17]
        // }
        let len = self.origin.read(buf)?;
        let audio = &mut buf[..len];

        let pub_key = KuGou::get_pub_key(self.pos..self.pos + len as u64);

        for (byte, i) in audio.iter_mut().zip(self.pos..self.pos + len as u64) {
            let own_key = self.own_key[(i % self.own_key.len() as u64) as usize] ^ *byte;
            let own_key = own_key ^ (own_key & 0x0f) << 4;

            let pub_key = PUB_KEY_MEND[(i % PUB_KEY_MEND.len() as u64) as usize]
                ^ pub_key[(i / KuGou::PUB_KEY_LEN_MAGNIFICATION) as usize
                    - (self.pos / KuGou::PUB_KEY_LEN_MAGNIFICATION) as usize];
            let pub_key = pub_key ^ (pub_key & 0xf) << 4;
            *byte = own_key ^ pub_key;
        }

        self.pos += len as u64;
        Ok(len)
    }
}

struct Bytes<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> Bytes<'a> {
    fn new(data: &[u8]) -> Bytes {
        Bytes { data, pos: 0 }
    }
}

impl Read for Bytes<'_> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let len = std::cmp::min(buf.len(), self.data.len() - self.pos);

        buf[..len].copy_from_slice(&self.data[self.pos..self.pos + len]);
        self.pos += len;

        Ok(len)
    }
}

#[test]
fn test_decode() {
    let mut decoder = KuGou::new(std::fs::File::open("src/assets/test_kugou_kgm.dat").unwrap());
    let mut right_file = std::fs::File::open("src/assets/test_kugou_kgm_right.dat").unwrap();

    let mut audio = Vec::new();
    let mut right_dat = Vec::new();

    decoder.read_to_end(&mut audio).unwrap();
    right_file.read_to_end(&mut right_dat).unwrap();

    println!("{} {}", audio.len(), right_dat.len());

    assert!(audio == right_dat);
}
