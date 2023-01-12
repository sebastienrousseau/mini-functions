#[cfg(test)]
mod tests {

    extern crate mini_functions_games;
    use self::mini_functions_games::Deck;

    #[test]

    fn test_has_winning_hand() {
        let card1 = "2 of Hearts (♥)".to_string();
        let card2 = "3 of Hearts (♥)".to_string();
        let card3 = "4 of Hearts (♥)".to_string();
        let deck = Deck::new();
        assert_eq!(deck.has_winning_hand(card1, card2, card3), true);
    }
}
