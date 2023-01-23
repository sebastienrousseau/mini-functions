#[cfg(test)]
mod tests {
    pub use common::Constants;
    pub use common::Words;
    use common::*;
    #[test]
    fn test_constant() {
        let constants = Constants.constant();
        assert_eq!(constants[0].0, "EULER_CONSTANT");
        assert_eq!(constants[0].1, EULER_CONSTANT.to_string());
        assert_eq!(constants[1].0, "HASH_ALGORITHM");
        assert_eq!(constants[1].1, HASH_ALGORITHM.to_string());
        assert_eq!(constants[2].0, "HASH_COST");
        assert_eq!(constants[2].1, HASH_COST.to_string());
        assert_eq!(constants[3].0, "HASH_LENGTH");
        assert_eq!(constants[3].1, HASH_LENGTH.to_string());
        assert_eq!(constants[4].0, "PHI_CONSTANT");
        assert_eq!(constants[4].1, PHI_CONSTANT.to_string());
        assert_eq!(constants[5].0, "PI_CONSTANT");
        assert_eq!(constants[5].1, PI_CONSTANT.to_string());
        assert_eq!(constants[6].0, "PLANCK_CONSTANT");
        assert_eq!(constants[6].1, PLANCK_CONSTANT.to_string());
        assert_eq!(constants[7].0, "SQRT5_CONSTANT");
        assert_eq!(constants[7].1, SQRT5_CONSTANT.to_string());
    }
}
