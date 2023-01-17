extern crate mini_functions_md5;
use self::mini_functions_md5::MD5;

pub trait Digest {
    fn reset(&mut self) -> &mut Self;
    fn update(&mut self, value: &[u8]) -> &mut Self;

    fn hexdigest(value: &str) -> String;
}
