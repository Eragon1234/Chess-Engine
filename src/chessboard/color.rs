pub enum Color {
    White,
    Black,
}

impl From<char> for Color {
    fn from(value: char) -> Self {
        match value {
            'w' => Color::White,
            'b' => Color::Black,
            _ => panic!("Invalid color: {}", value)
        }
    }
}
