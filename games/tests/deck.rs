#[cfg(test)]
mod tests {

    extern crate games;
    use self::games::Deck;

    #[test]

    fn test_has_winning_hand() {
        let card1 = "2 of Hearts (♥)".to_string();
        let card2 = "3 of Hearts (♥)".to_string();
        let card3 = "4 of Hearts (♥)".to_string();
        let deck = Deck::new();
        assert!(deck.has_winning_hand(card1, card2, card3));
    }
}
