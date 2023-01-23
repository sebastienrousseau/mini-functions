#[cfg(test)]
mod tests {
    extern crate md5;
    use md5::*;

    #[test]
    fn test_md5_0() {
        let digest = MD5::hexdigest("");
        assert_eq!(digest, "d41d8cd98f00b204e9800998ecf8427e");
    }

    #[test]
    fn test_md5_1() {
        let digest = MD5::hexdigest("a");
        assert_eq!(digest, "0cc175b9c0f1b6a831c399e269772661");
    }

    #[test]
    fn test_md5_2() {
        let digest = MD5::hexdigest("abc");
        assert_eq!(digest, "900150983cd24fb0d6963f7d28e17f72");
    }

    #[test]
    fn test_md5_3() {
        let digest = MD5::hexdigest("message digest");
        assert_eq!(digest, "f96b697d7cb7938d525a2f31aaf161d0");
    }

    #[test]
    fn test_md5_4() {
        let digest = MD5::hexdigest("abcdefghijklmnopqrstuvwxyz");
        assert_eq!(digest, "c3fcd3d76192e4007dfb496cca67e13b");
    }

    #[test]
    fn test_md5_5() {
        let digest =
            MD5::hexdigest("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789");
        assert_eq!(digest, "d174ab98d277d9f5a5611c2c9f419d9f");
    }

    #[test]
    fn test_md5_6() {
        let digest = MD5::hexdigest(
            "12345678901234567890123456789012345678901234567890123456789012345678901234567890",
        );
        assert_eq!(digest, "57edf4a22be3c955ac49da2e2107b67a");
    }

    #[test]
    fn test_update_md5_many_times() {
        let digest = MD5::new()
            .update(b"a")
            .update(b"b")
            .update(b"c")
            .finalize()
            .to_string();
        assert_eq!(digest, "900150983cd24fb0d6963f7d28e17f72");
    }

    #[test]
    fn test_reset() {
        let digest = MD5::new().update(b"a").reset().finalize().to_string();
        assert_eq!(digest, "d41d8cd98f00b204e9800998ecf8427e");
    }

    #[test]
    fn test_to_string_before_finalize() {
        let digest = MD5::new().update(b"a").to_string();
        // Is this acceptable? Or make it an Option that can be None?
        assert_eq!(digest, "00000000000000000000000000000000");
    }

    #[test]
    fn update_with_len() {
        let digest = MD5::new().update_with_len(b"abc", 3).finalize().to_string();
        assert_eq!(digest, "900150983cd24fb0d6963f7d28e17f72");
    }
    #[test]
    fn update_file() {
        let digest = MD5::new()
            .update_file("../update.txt")
            .finalize()
            .to_string();
        assert_eq!(digest, "47353a0e5ed2e1e0d57213a39e9bb7c4");
    }
    #[test]
    fn hexdigest_file() {
        let digest = MD5::hexdigest_file("../update.txt");
        assert_eq!(digest, "47353a0e5ed2e1e0d57213a39e9bb7c4");
    }
    #[test]
    fn reset_file() {
        let digest = MD5::new()
            .update_file("../update.txt")
            .reset()
            .finalize()
            .to_string();
        assert_eq!(digest, "d41d8cd98f00b204e9800998ecf8427e");
    }
    #[test]
    fn transform() {
        let digest = MD5::new()
            .update_file("../update.txt")
            .finalize()
            .to_string();
        assert_eq!(digest, "47353a0e5ed2e1e0d57213a39e9bb7c4");
    }
}
