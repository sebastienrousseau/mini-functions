#[cfg(test)]
mod tests {

    extern crate mini_functions_random;
    use self::mini_functions_random::Random;

    // #[test]
    // fn test_bool() {
    //     let bool = Random::bool();
    //     assert!(bool);
    // }

    #[test]
    fn test_new() {
        let mut rng = Random::new();
        assert!(rng.random() > 0);
    }

    #[test]
    fn test_default() {
        let mut rng = Random::default();
        assert!(rng.random() > 0);
    }

    #[test]
    fn test_random() {
        let mut rng = Random::new();
        let first = rng.random();
        let second = rng.random();
        assert_ne!(first, second);
    }

    #[test]
    fn test_bytes() {
        let bytes = Random::bytes(0);
        assert_eq!(bytes.len(), 0);

        let bytes = Random::bytes(10);
        assert_eq!(bytes.len(), 10);

        let bytes = Random::bytes(100);
        assert_eq!(bytes.len(), 100);
    }

    #[test]
    fn test_pseudo() {
        let mut rng = Random::new();
        let first = rng.pseudo();
        let second = rng.pseudo();
        assert_ne!(first, second);
    }

    #[test]
    fn test_display() {
        let rng = Random::new();
        let s = format!("{}", rng);
        assert!(s.starts_with("Random { seed: "));
        assert!(s.ends_with('}'));
    }

    #[test]
    fn test_random_sum_is_positive() {
        let mut rng = Random::new();
        let mut sum = 0;
        for _ in 0..100 {
            sum += rng.random();
        }
        assert!(sum > 0);
    }
    #[test]
    fn test_pseudo_sum_is_positive() {
        let mut rng = Random::new();
        let mut sum = 0;
        for _ in 0..100 {
            sum += rng.pseudo();
        }
        assert!(sum > 0);
    }
    #[test]
    fn test_random_is_within_range() {
        let mut rng = Random::new();
        let mut sum = 0;
        for _ in 0..100 {
            sum += rng.random();
        }
        assert!(sum > 0);
    }
    #[test]
    fn test_pseudo_is_within_range() {
        let mut rng = Random::new();
        let mut sum = 0;
        for _ in 0..100 {
            sum += rng.pseudo();
        }
        assert!(sum > 0);
    }
    #[test]
    fn integration_test_pseudo_and_random_are_different() {
        let mut rng = Random::new();
        let mut pseudo_sum = 0;
        let mut random_sum = 0;
        for _ in 0..100 {
            pseudo_sum += rng.pseudo();
            random_sum += rng.random();
        }
        assert_ne!(pseudo_sum, random_sum);
    }

    #[test]
    fn integration_test_random_values_are_within_range() {
        let mut rng = Random::new();
        for _ in 0..100 {
            let random = rng.random_range(0, std::u32::MAX);
            assert!(random < std::u32::MAX);
        }
    }

    #[test]
    fn integration_test_pseudo_values_are_within_range() {
        let mut rng = Random::new();
        for _ in 0..100 {
            let pseudo = rng.random_range(0, std::u32::MAX);
            assert!(pseudo < std::u32::MAX);
        }
    }

    #[test]
    fn integration_test_random_values_are_distributed_evenly() {
        let mut rng = Random::new();
        let mut counts = [0; 100];
        for _ in 0..100000 {
            let random = rng.random();
            let index = (random % 100) as usize;
            counts[index] += 1;
        }
        for count in counts.iter() {
            assert!(*count >= 0);
        }
    }
    #[test]
    fn integration_test_pseudo_values_are_distributed_evenly() {
        let mut rng = Random::new();
        let mut counts = [0; 100];
        for _ in 0..100000 {
            let pseudo = rng.pseudo();
            let index = (pseudo % 100) as usize;
            counts[index] += 1;
        }
        for count in counts.iter() {
            assert!(*count >= 0);
        }
    }
}
