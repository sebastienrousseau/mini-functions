#[cfg(test)]
mod tests {
    pub use common::Words;

    #[test]
    fn test_words_list() {
        let words = Words::new();
        let words_list = words.words_list();
        assert_eq!(words_list[0], "Adam");
        assert_eq!(words_list[1], "Afghan");
        assert_eq!(words_list[2], "Alaska");
    }

    #[test]
    fn test_default_words() {
        let words = Words::default();
        let words_list = words.words_list();
        assert_eq!(words_list[0], "Adam");
        assert_eq!(words_list[1], "Afghan");
        assert_eq!(words_list[2], "Alaska");
    }
}
