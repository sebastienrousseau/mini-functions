#[cfg(test)]

// TODO: Add more tests to bring the code coverage to 100%
mod tests {
    use common::constants::*;
    use pwd::Password;
    use random::Random;

    #[test]
    fn test_password() {
        let password = Password::new(3, "-", SPECIAL_CHARS.to_vec());
        assert_eq!(password.passphrase().split('-').count(), 3);
    }

    #[test]
    fn test_default() {
        let default_result = Password::default();
        let expected_result = Password::new(4, "-", SPECIAL_CHARS.to_vec());
        assert_ne!(default_result, expected_result);
    }

    #[test]
    fn test_set_password() {
        let mut password = Password::new(3, "-", SPECIAL_CHARS.to_vec());
        password.set_passphrase("test");
        assert_eq!(password.passphrase(), "test");
    }

    #[test]
    fn test_password_entropy() {
        let password = Password::new(4, "-", vec!['!', '@', '#']);
        let entropy = password.entropy();
        assert!(entropy > 71.0);
    }

    #[test]
    fn test_password_hash_length() {
        let password = Password::new(4, "-", vec!['!', '@', '#']);
        let hash_length = password.hash_length();
        assert_eq!(hash_length, 64);
    }

    #[test]
    fn test_password_is_empty() {
        let password = Password::new(0, "-", vec!['!', '@', '#']);
        let is_empty = password.is_empty();
        assert!(is_empty, "{}", password.passphrase());
    }

    #[test]
    fn test_password_length() {
        let mut password = Password::new(4, "-", SPECIAL_CHARS.to_vec());
        password.set_passphrase("M1n1Funct1()ns-N3wP@s5phr4s3-Ex@mpl3");
        assert_eq!(password.password_length(), 36);
    }

    #[test]
    fn test_password_default() {
        let len = 4;
        let password = Password::new(len, "-", SPECIAL_CHARS.to_vec());
        assert!(password.passphrase().split('-').count() == len.into());
    }

    #[test]
    fn test_password_display() {
        let mut password = Password::new(4, "-", SPECIAL_CHARS.to_vec());
        password.set_passphrase("M1n1Funct1()ns-N3wP@s5phr4s3-Ex@mpl3");
        assert_eq!(
            format!("{password}"),
            "M1n1Funct1()ns-N3wP@s5phr4s3-Ex@mpl3"
        );
    }

    #[test]
    fn test_wordlist() {
        let words: Vec<String> = vec!["mini".to_string(), "functions".to_string()];
        let mut rng = Random::default();
        while words.len() < 2 {
            if let Some(w) = Random::choose(&mut rng, words.as_slice()) {
                assert!(words.contains(w));
            } else {
                assert!(words.is_empty());
            };
        }
    }
    #[test]
    fn test_wordlist_is_empty() {
        let word = "";
        assert!(word.is_empty());
    }
}
