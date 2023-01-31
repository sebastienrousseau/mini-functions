#[cfg(test)]

// TODO: Add more tests to bring the code coverage to 100%
mod tests {
    extern crate md5;
    use md5::*;

    #[test]
    fn test_md5_reset() {
        let mut md5 = MD5::new();
        md5.update(b"test");
        md5.reset();
        let digest = md5::MD5::hexdigest("");
        assert_eq!(digest, "d41d8cd98f00b204e9800998ecf8427e");
    }

    #[test]
    fn test_md5_update() {
        let mut md5 = MD5::new();
        md5.update(b"test");
        let digest = md5::MD5::hexdigest("");
        assert_eq!(digest, "d41d8cd98f00b204e9800998ecf8427e");
    }

    #[test]
    fn test_md5_hexdigest() {
        let mut md5 = MD5::new();
        md5.update(b"test");
        let digest = md5::MD5::hexdigest("");
        assert_eq!(digest, "d41d8cd98f00b204e9800998ecf8427e");
    }
}
