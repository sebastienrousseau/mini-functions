#[cfg(test)]
mod tests {
    extern crate hsh;
    use self::hsh::Hash;

    #[test]
    fn test_hash_fmt() {
        let mut hash = Hash::new();
        hash.set_password("password");
        hash.set_hash("hash");
        assert_eq!(format!("{hash}"), "Hash { password: password, hash: hash }");
    }

    #[test]
    fn test_hash_password() {
        let mut hash = Hash::new();
        hash.set_password("password");
        hash.set_hash("hash");
        assert_eq!(hash.password, "password");
    }

    #[test]
    fn test_hash_hash() {
        let mut hash = Hash::new();
        hash.set_password("password");
        hash.set_hash("hash");
        assert_eq!(hash.hash, "hash");
    }

    #[test]
    fn test_new() {
        let hash = Hash::new();
        assert_eq!(hash.password(), "");
        assert_eq!(hash.hash(), "");
    }

    #[test]
    fn test_default() {
        let hash = Hash::default();
        assert_eq!(hash.password(), "");
        assert_eq!(hash.hash(), "");
    }

    #[test]
    fn test_generate_hash() {
        let hash = Hash::new();
        let hash = hash.generate_hash();
        assert_eq!(hash.len(), 64);
    }

    #[test]
    fn test_verify() {
        let mut hash = Hash::new();
        hash.set_password("password");
        assert!(hash.verify(
            "7f2611ba158b6dcea4a69c229c303358c5e04493abeadee106a4bfa464d55787",
            "password",
        ));
    }

    #[test]
    fn test_set_password() {
        let mut hash = Hash::new();
        hash.set_password("password");
        assert_eq!(hash.password(), "password");
    }
    #[test]
    fn test_set_hash() {
        let mut hash = Hash::new();
        hash.set_hash("hash");
        assert_eq!(hash.hash(), "hash");
    }

    #[test]
    fn test_password() {
        let mut hash = Hash::new();
        hash.set_password("password");
        assert_eq!(hash.password(), "password");
    }
    #[test]
    fn test_password_length() {
        let mut hash = Hash::new();
        hash.set_password("password");
        assert_eq!(hash.password_length(), 8);
    }
    #[test]
    fn test_hash() {
        let mut hash = Hash::new();
        hash.set_hash("hash");
        assert_eq!(hash.hash(), "hash");
    }
    #[test]
    fn test_hash_length() {
        let mut hash = Hash::new();
        hash.set_hash("hash");
        assert_eq!(hash.hash_length(), 4);
    }
    #[test]
    fn test_entropy() {
        let mut hash = Hash::new();
        hash.set_password("password");
        assert!(hash.entropy() > 0.0);
    }
}
