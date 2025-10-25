mod kugou;

use std::io::Read;

use kugou::KuGou;

pub fn new<'a>(data: impl Read + 'a) -> Option<impl Decoder<'a>> {
    KuGou::try_new(data)
}

pub trait Decoder<'a>: Sized + Read {
    #[allow(dead_code)]
    fn new(origin: impl Read + 'a) -> Self;
    fn try_new(origin: impl Read + 'a) -> Option<Self>;
    #[allow(dead_code)]
    fn decodeable_length_interval() -> (u64, u64);
}
