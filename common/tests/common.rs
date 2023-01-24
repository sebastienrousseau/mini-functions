#[cfg(test)]
mod tests {
    pub use common::Words;
    use common::*;

    #[test]
    fn test_new() {
        let common = Common::new();
        assert_eq!(common.constants().constant().len(), 9);
    }

    #[test]
    fn test_constants() {
        let common = Common::new();
        let constants = common.constants();
        assert_eq!(constants.constant().len(), 9);
    }

    #[test]
    fn test_words() {
        let common = Common::new();
        let words = common.words();
        assert_eq!(words.words_list().len(), 4096);
    }

    #[test]
    fn test_default() {
        let common = Common::default();
        assert_eq!(common.constants().constant().len(), 9);
    }
}
