use create::colors::Color;
use create::values::Value;

pub struct Card {
    pub color: Color,
    pub value: Value,
}

impl Card {
    pub fn new(color: Color, value: Value) -> Self {
        Card { color, value }
    }
}
