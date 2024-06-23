pub struct Hand {
    cards_in_hand: vector<Card>,
}
impl Hand {
    pub fn new() -> self {
        let mut cards_in_hand = Vec::new();
    }

    pub fn add(&mut self, new_card: Card) {
        self.cards_in_hand.add(new_card);
    }

    pub fn values(self) -> u32 {}
}
