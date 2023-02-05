#[cfg(test)]
mod tests {
    pub use cmn::Words;

    #[test]
    fn test_words_list() {
        let words = Words::new();
        let words_list = words.words_list();
        assert_eq!(words_list[0], "aboard");
        assert_eq!(words_list[1], "abode");
        assert_eq!(words_list[2], "abort");
    }

    #[test]
    fn test_default_words() {
        let words = Words::default();
        let words_list = words.words_list();
        assert_eq!(words_list[0], "aboard");
        assert_eq!(words_list[1], "abode");
        assert_eq!(words_list[2], "abort");
    }
}
