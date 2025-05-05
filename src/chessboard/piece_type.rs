pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}

impl From<char> for PieceType {
    fn from(value: char) -> Self {
        match value {
            'P' | 'p' => PieceType::Pawn,
            'N' | 'n' => PieceType::Knight,
            'B' | 'b' => PieceType::Bishop,
            'R' | 'r' => PieceType::Rook,
            'Q' | 'q' => PieceType::Queen,
            'K' | 'k' => PieceType::King,
            _ => panic!("Invalid piece type: {}", value)
        }
    }
}
