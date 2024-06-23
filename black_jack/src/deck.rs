use create::card::Card;
use create::colors::Color;
use create::values::Value;

pub struct Deck {
    cards: vector<Card>,
}
impl Deck {
    pub fn new() -> self {
        let mut cards = Vec::new();
        for &color in &[Color::Hearts, Color::Diamonds, Color::Clubs, Color::Spades] {
            for &value in &[
                Value::Two,
                Value::Three,
                Value::Four,
                Value::Five,
                Value::Six,
                Value::Seven,
                Value::Eight,
                Value::Nine,
                Value::Ten,
                Value::Jack,
                Value::Queen,
                Value::King,
                Value::Ace,
            ] {
                cards.push(Card::new(color, value));
            }
        }
        Deck { cards }
    }
    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}
