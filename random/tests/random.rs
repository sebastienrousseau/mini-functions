#[cfg(test)]
mod tests {

    extern crate random;
    use self::random::Random;

    const N: usize = 624;

    #[test]
    fn test_bool() {
        let mut rng = Random::new();
        let b = Random::bool(&mut rng);
        assert!(b);
    }
    #[test]
    fn test_bytes() {
        let mut rng = Random::new();
        let bytes = Random::bytes(&mut rng, 0);
        assert_eq!(bytes.len(), 0);

        let bytes = Random::bytes(&mut rng, 10);
        assert_eq!(bytes.len(), 10);

        let bytes = Random::bytes(&mut rng, 100);
        assert_eq!(bytes.len(), 100);
    }
    #[test]
    fn test_char() {
        let mut rng = Random::new();
        let c = Random::char(&mut rng);
        assert!(('a'..='z').contains(&c));
    }
    #[test]
    fn test_choose() {
        let mut rng = Random::new();
        let values = vec![1, 2, 3, 4, 5];
        let value = Random::choose(&mut rng, &values);
        assert!(value.is_some());
        assert!(value.unwrap() >= &1 && value.unwrap() <= &5);
    }
    #[test]
    fn test_float() {
        let mut rng = Random::new();
        let f = Random::float(&mut rng);
        assert!((0.0..=1.0).contains(&f));
    }
    #[test]
    fn test_int() {
        let mut rng = Random::new();
        let i = rng.int(0, 10);
        assert!(i >= 0 && i <= 10);
    }
    #[test]
    fn test_pseudo() {
        let mut rng = Random::new();
        let p = Random::pseudo(&mut rng);
        assert!(p < 4294967295);
    }
    #[test]
    fn test_range() {
        let mut rng = Random::new();
        let r = Random::range(&mut rng, 0, 10);
        assert!(r >= 0 && r <= 10);
    }
    #[test]
    pub fn test_new() {
        let rng = Random::new();
        assert!(rng.mti <= N);
        assert!(rng.mt[0] > 0);
    }
    #[test]
    fn test_rand() {
        let mut rng = Random::new();
        let r = Random::rand(&mut rng);
        assert!(r < 4294967295);
    }
    #[test]
    fn test_random_range() {
        let mut rng = Random::new();
        let r = Random::random_range(&mut rng, 0, 10);
        assert!(r <= 10);
    }
    #[test]
    fn test_seed() {
        let mut rng = Random::new();
        Random::seed(&mut rng, 0);
        assert!(rng.mti <= N);
        assert!(rng.mt.iter().any(|&x| x != 0));
    }
    #[test]
    fn test_twist() {
        let mut rng = Random::new();
        Random::seed(&mut rng, 0);
        Random::twist(&mut rng);
        assert!(rng.mti <= N);
        assert!(rng.mt.iter().any(|&x| x != 0));
    }
}
