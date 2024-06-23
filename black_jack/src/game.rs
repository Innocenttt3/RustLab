pub struct Game_builder {
    deck: Deck,
    hand: Hand,
}

impl Game_builder {
    pub fn new() -> Self {
        let mut deck = Deck::new();
        deck.shuffle();
        let mut hand = Hand::new();
    }
}
