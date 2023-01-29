#[cfg(test)]
mod tests {

    extern crate games;
    use self::games::Deck;

    #[test]
    fn test_determine_winnings() {
        let mut deck = Deck::new();
        deck.bet = 10;
        deck.cards = vec![1, 2, 3];
        deck.determine_winnings();
        assert_eq!(deck.balance, 120);
        assert_eq!(deck.bet, 10);

        deck.cards = vec![1, 2, 4];
        deck.determine_winnings();
        assert_eq!(deck.balance, 140);
        assert_eq!(deck.bet, 10);
    }

    #[test]
    fn test_display_balance() {
        let deck = Deck {
            balance: 100,
            ..Deck::default()
        };
        deck.display_balance();
        // check that the output is "💰 Your current balance is: $100"
    }

    #[test]
    fn test_display_bet() {
        let deck = Deck {
            bet: 50,
            ..Deck::default()
        };
        deck.display_bet();
        // check that the output is "💰 Your current bet is: $50"
    }

    #[test]
    fn test_draw() {
        let mut deck = Deck {
            cards: vec![1, 2, 3],
            ..Deck::default()
        };
        let card1 = deck.draw();
        assert_eq!(card1, Some("5 of Spades (♠)".to_string()));
        let card2 = deck.draw();
        assert_eq!(card2, Some("4 of Spades (♠)".to_string()));
        let card3 = deck.draw();
        assert_eq!(card3, Some("3 of Spades (♠)".to_string()));
        let card4 = deck.draw();
        assert_eq!(card4, None);
    }

    #[test]
    fn test_has_winning_hand() {
        let deck = Deck::default();
        let card1 = "2 of Spades (♠)".to_string();
        let card2 = "3 of Spades (♠)".to_string();
        let card3 = "4 of Spades (♠)".to_string();
        let result = deck.has_winning_hand(card1, card2, card3);
        assert!(result);

        let card1 = "2 of Spades (♠)".to_string();
        let card2 = "3 of Clubs (♣)".to_string();
        let card3 = "4 of Spades (♠)".to_string();
        let result = deck.has_winning_hand(card1, card2, card3);
        assert!(!result);
    }

    #[test]
    fn test_new() {
        let deck = Deck::new();
        assert_eq!(deck.cards.len(), 52);
        assert_eq!(deck.balance, 100);
        assert_eq!(deck.bet, 0);
    }
    #[test]
    fn test_place_bet() {
        let mut deck = Deck::new();
        deck.place_bet(5);
        assert_eq!(deck.balance, 95);
        assert_eq!(deck.bet, 5);
        deck.place_bet(105);
    }
}
