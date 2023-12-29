#[cfg(test)]

// TODO: Add more tests to bring the code coverage to 100%
mod tests {
    extern crate mdg;
    use std::cmp;

    use mdg::{Digest, MD5};

    extern crate cjwt;
    use self::cjwt::{Algorithm, JWT};

    #[test]
    fn test_mdg_0() {
        let digest = MD5::hexdigest("");
        assert_eq!(digest, "d41d8cd98f00b204e9800998ecf8427e");
    }

    #[test]
    fn test_mdg_1() {
        let digest = MD5::hexdigest("a");
        assert_eq!(digest, "0cc175b9c0f1b6a831c399e269772661");
    }

    #[test]
    fn test_mdg_2() {
        let digest = MD5::hexdigest("abc");
        assert_eq!(digest, "900150983cd24fb0d6963f7d28e17f72");
    }

    #[test]
    fn test_mdg_3() {
        let digest = MD5::hexdigest("message digest");
        assert_eq!(digest, "f96b697d7cb7938d525a2f31aaf161d0");
    }

    #[test]
    fn test_mdg_4() {
        let digest = MD5::hexdigest("abcdefghijklmnopqrstuvwxyz");
        assert_eq!(digest, "c3fcd3d76192e4007dfb496cca67e13b");
    }

    #[test]
    fn test_mdg_5() {
        let digest =
            MD5::hexdigest("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789");
        assert_eq!(digest, "d174ab98d277d9f5a5611c2c9f419d9f");
    }

    #[test]
    fn test_mdg_6() {
        let digest = MD5::hexdigest(
            "12345678901234567890123456789012345678901234567890123456789012345678901234567890",
        );
        assert_eq!(digest, "57edf4a22be3c955ac49da2e2107b67a");
    }

    #[test]
    fn test_update_mdg_many_times() {
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
        let digest = MD5::new().update_file("update.txt").finalize().to_string();
        assert_eq!(digest, "47353a0e5ed2e1e0d57213a39e9bb7c4");
    }
    #[test]
    fn hexdigest_file() {
        let digest = MD5::hexdigest_file("update.txt");
        assert_eq!(digest, "47353a0e5ed2e1e0d57213a39e9bb7c4");
    }
    #[test]
    fn reset_file() {
        let digest =
            MD5::new()
                .update_file("update.txt")
                .reset()
                .finalize()
                .to_string();
        assert_eq!(digest, "d41d8cd98f00b204e9800998ecf8427e");
    }
    #[test]
    fn transform() {
        let digest = MD5::new().update_file("update.txt").finalize().to_string();
        assert_eq!(digest, "47353a0e5ed2e1e0d57213a39e9bb7c4");
    }
    #[test]
    fn test_algorithm_to_string() {
        let hs256 = Algorithm::HS256;
        assert_eq!(hs256.to_string(), "HS256");

        let rs512 = Algorithm::RS512;
        assert_eq!(rs512.to_string(), "RS512");
    }

    #[test]
    fn test_default_algorithm() {
        let default_alg = Algorithm::default();
        assert_eq!(default_alg, Algorithm::HS256);
    }

    #[test]
    fn test_default_jwt() {
        let default_jwt = JWT::default();
        assert_eq!(default_jwt.header.alg, Some(Algorithm::HS256));
        assert_eq!(default_jwt.token, String::new());
    }
    #[test]
    fn test_reset_file() {
        let mut mdg = MD5::new();
        mdg.finalize();
        mdg.reset();
        assert_eq!(mdg.to_string(), "00000000000000000000000000000000");
    }
    #[test]
    fn test_default() {
        let mut mdg = MD5::default();
        mdg.finalize();
        mdg.reset();
        assert_eq!(mdg.to_string(), "00000000000000000000000000000000");
    }

    #[test]
    fn test_update_with_len() {
        let mut md5 = MD5::new();
        let data = b"test data";
        let nbytes = data.len();

        // Test updating with non-empty data
        md5.update_with_len(data, nbytes);
        assert_eq!(md5.count, [72, 0]);
        assert_eq!(md5.buffer[0..nbytes], data[..]);
        // Check the state of the md5 after the update
        let state = md5.state;
        assert_ne!(state, [0; 4]);
        md5.reset();

        // Test updating with empty data
        md5.update_with_len(b"", 0);
        assert_eq!(md5.count, [0, 0]);
        // Check the state of the md5 after the update
        let state = md5.state;
        assert_ne!(state, [0; 4]);
        md5.reset();

        // Test updating with data that is smaller than part_len
        let data = b"test";
        let nbytes = data.len();
        md5.update_with_len(data, nbytes);
        assert_eq!(md5.count, [32, 0]);
        assert_eq!(md5.buffer[0..nbytes], data[..]);
        // Check the state of the md5 after the update
        let state = md5.state;
        assert_ne!(state, [0; 4]);
        md5.reset();

        // Test updating with data that is larger than part_len
        let data = b"test data test data test data";
        let nbytes = data.len();
        md5.update_with_len(data, nbytes);
        assert_eq!(md5.count, [232, 0]);
        // Check the state of the md5 after the update
        let state = md5.state;
        assert_ne!(state, [0; 4]);
        md5.reset();

        // Test updating with data that results in overflow of count[0]
        let data = b"test data";
        let nbytes = data.len();
        md5.count[0] = u32::max_value();
        md5.update_with_len(data, nbytes);
        assert_eq!(md5.count, [71, 1]);
        // Check the state of the md5 after the update
        let state = md5.state;
        assert_ne!(state, [0; 4]);
        md5.reset();
    }
    #[test]
    fn test_update_with_len_large_data() {
        let mut md5 = MD5::new();
        let data = vec![1; 128];
        let nbytes = data.len();

        md5.update_with_len(&data, nbytes);
        assert_eq!(md5.count, [1024, 0]);

        let buffer_len = cmp::min(md5.buffer.len(), nbytes);
        assert_eq!(md5.buffer[0..buffer_len], data[..buffer_len]);

        let buf = md5.buffer[0..buffer_len].to_vec();
        md5.transform(&buf);
        assert!(buffer_len >= 64);
        assert_eq!(md5.count, [1024, 0]);

        let state = md5.state;
        assert_ne!(state, [0; 4]);
        md5.reset();

        // Check the state of the md5 after each iteration of the while loop
        let state1 = md5.state;
        assert_ne!(state1, [0; 4]);

        let state2 = md5.state;
        assert_ne!(state2, [0; 4]);
        assert_eq!(state1, state2);

        md5.reset();
    }
    #[test]
    fn test_transform_with_sufficient_buffer_length() {
        const NBYTES: usize = 1024;

        let mut md5 = MD5::new();
        md5.count = [NBYTES as u32, 0];
        let buffer = (0..NBYTES).map(|i| i as u8).collect::<Vec<u8>>();

        let mut i = 0;
        while i < NBYTES {
            if NBYTES - i >= 64 {
                let buf = &buffer[i..(i + 64)].to_vec();
                md5.transform(buf);
                i += 64;
            } else {
                let part_len = NBYTES - i;
                let buf = &buffer[i..(i + part_len)].to_vec();
                md5.transform(buf);
                i += part_len;
            }
        }

        assert_eq!(i, NBYTES);
        assert_eq!(md5.count, [1024, 0]);
    }
    #[test]
    fn test_transform_with_insufficient_buffer_length() {
        const NBYTES: usize = 1024;
        let mut md5 = MD5::new();
        md5.count = [NBYTES as u32, 0];
        let buffer = (0..NBYTES).map(|i| i as u8).collect::<Vec<u8>>();

        let mut i = 0;
        while i < NBYTES {
            if NBYTES - i >= 64 {
                let buf = &buffer[i..(i + 64)].to_vec();
                md5.transform(buf);
                i += 64;
            } else {
                let part_len = NBYTES - i;
                let buf = &buffer[i..(i + part_len)].to_vec();
                md5.transform(buf);
                i += part_len;
            }
        }
        assert_eq!(i, NBYTES);
        assert_eq!(md5.count, [1024, 0]);
        md5.reset();
    }

    // #[test]
    // fn test_transform_with_insufficient_buffer_length() {
    //     const NBYTES: usize = 63;

    //     let mut md5 = MD5::new();
    //     md5.count = [NBYTES as u32, 0];
    //     let buffer = (0..NBYTES).map(|i| i as u8).collect::<Vec<u8>>();

    //     let mut i = 0;
    //     let part_len = NBYTES - i;
    //     while i < NBYTES {
    //         if NBYTES >= part_len {
    //             let buf = &buffer[i..(i + 64)].to_vec();
    //             md5.transform(buf);

    //             while i < NBYTES - part_len {
    //                 if NBYTES - i >= 64 {
    //                     let buf = md5.buffer[i..i + 64].to_vec();
    //                     md5.transform(&buf);
    //                     i += 64;
    //                 } else {
    //                     let buf = md5.buffer[i..NBYTES].to_vec();
    //                     md5.transform(&buf);
    //                     break;
    //                 }
    //             }
    //         }
    //     }

    //     assert_eq!(i, NBYTES);
    //     assert_eq!(md5.count, [63, 0]);
    // }
}
