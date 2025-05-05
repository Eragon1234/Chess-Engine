use crate::chessboard::color::Color;

pub enum CastlingSide {
    KingSide,
    QueenSide,
}

pub struct CastlingRights {
    white_kingside: bool,
    white_queenside: bool,
    black_kingside: bool,
    black_queenside: bool,
}

impl CastlingRights {
    pub fn new() -> Self {
        CastlingRights {
            white_kingside: false,
            white_queenside: false,
            black_kingside: false,
            black_queenside: false,
        }
    }

    pub fn has_right(&self, color: Color, side: CastlingSide) -> bool {
        match (color, side) {
            (Color::White, CastlingSide::KingSide) => self.white_kingside,
            (Color::White, CastlingSide::QueenSide) => self.white_queenside,
            (Color::Black, CastlingSide::KingSide) => self.black_kingside,
            (Color::Black, CastlingSide::QueenSide) => self.black_queenside,
        }
    }

    pub fn remove_right(&mut self, color: Color, side: CastlingSide) {
        match (color, side) {
            (Color::White, CastlingSide::KingSide) => self.white_kingside = false,
            (Color::White, CastlingSide::QueenSide) => self.white_queenside = false,
            (Color::Black, CastlingSide::KingSide) => self.black_kingside = false,
            (Color::Black, CastlingSide::QueenSide) => self.black_queenside = false,
        }
    }
}

impl From<&str> for CastlingRights {
    fn from(s: &str) -> Self {
        let mut rights = CastlingRights::new();

        if s == "-" {
            return rights;
        }

        for c in s.chars() {
            match c {
                'K' => rights.white_kingside = true,
                'Q' => rights.white_queenside = true,
                'k' => rights.black_kingside = true,
                'q' => rights.black_queenside = true,
                _ => panic!("Invalid castling rights: {}", s),
            }
        }

        rights
    }
}
