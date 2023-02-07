#[cfg(test)]

// TODO: Add more tests to bring the code coverage to 100%
mod tests {
    extern crate mdg;

    use mdg::*;

    #[test]
    fn test_mdg_reset() {
        let mut mdg = MD5::new();
        mdg.update(b"test");
        mdg.reset();
        let digest = mdg::MD5::hexdigest("");
        assert_eq!(digest, "d41d8cd98f00b204e9800998ecf8427e");
    }

    #[test]
    fn test_mdg_update() {
        let mut mdg = MD5::new();
        mdg.update(b"test");
        let digest = mdg::MD5::hexdigest("");
        assert_eq!(digest, "d41d8cd98f00b204e9800998ecf8427e");
    }

    #[test]
    fn test_mdg_hexdigest() {
        let mut mdg = MD5::new();
        mdg.update(b"test");
        let digest = mdg::MD5::hexdigest("");
        assert_eq!(digest, "d41d8cd98f00b204e9800998ecf8427e");
    }
    #[test]
    fn test_reset_file() {
        let mut mdg = MD5::new();
        mdg.update(b"test");
        mdg.reset_file("file.txt");
        assert_eq!(mdg::MD5::hexdigest(""), "d41d8cd98f00b204e9800998ecf8427e");
    }
}
