use crate::chessboard::castling_rights::CastlingRights;
use crate::chessboard::color::Color;
use crate::chessboard::piece_type::PieceType;
use crate::chessboard::square::Square;

struct Bitboards {
    pieces: [u64; 6],
    colors: [u64; 2],
}

pub struct Chessboard {
    piece_placement: Bitboards,
    pub active_color: Color,
    pub castling_rights: CastlingRights,
    pub en_passant_square: Option<Square>,
    pub halfmove_clock: usize,
    pub fullmove_number: usize,
}

impl Chessboard {
    fn new() -> Chessboard {
        Chessboard {
            piece_placement: Bitboards {
                pieces: [0; 6],
                colors: [0; 2],
            },
            active_color: Color::White,
            castling_rights: CastlingRights::new(),
            en_passant_square: None,
            halfmove_clock: 0,
            fullmove_number: 0,
        }
    }

    fn set_square(&mut self, piece: PieceType, color: Color, square: Square) {
        let bit_mask = 1 << square as usize;
        self.piece_placement.pieces[piece as usize] |= bit_mask;
        self.piece_placement.colors[color as usize] |= bit_mask;
    }

    fn clear_square(&mut self, piece: PieceType, color: Color, square: Square) {
        let bit_mask = 1 << square as usize;
        self.piece_placement.pieces[piece as usize] &= !bit_mask;
        self.piece_placement.colors[color as usize] &= !bit_mask;
    }
}

impl From<&str> for Chessboard {
    /// Parses a string in FEN notation and returns a new Chessboard.
    fn from(value: &str) -> Self {
        let mut chessboard = Chessboard::new();

        let parts = value.split(' ').collect::<Vec<&str>>();
        assert_eq!(parts.len(), 6, "invalid fen: missing parts");
        
        let mut rank = 7;
        let mut file = 0;

        let piece_placement = parts[0];
        let active_color = parts[1].chars().next().expect("invalid fen: missing active color");
        let castling_rights = parts[2];
        let en_passant = parts[3];
        let halfmove = parts[4];
        let fullmove = parts[5];

        for c in piece_placement.chars() {
            match c {
                '1'..='8' => {
                    file += c.to_digit(10).unwrap() as usize;
                }
                '/' => {
                    rank -= 1;
                    file = 0;
                }
                _ => {
                    let square = Square::from((rank, file));
                    let piece = PieceType::from(c);
                    let color = if c.is_uppercase() {
                        Color::White
                    } else {
                        Color::Black
                    };

                    chessboard.set_square(piece, color, square);

                    file += 1;
                }
            }
        }

        chessboard.active_color = active_color.into();
        chessboard.castling_rights = castling_rights.into();
        chessboard.en_passant_square = match en_passant {
            "-" => None,
            _ => Some(Square::from(en_passant)),
        };
        chessboard.halfmove_clock = halfmove.parse().expect("invalid fen: invalid halfmove clock");
        chessboard.fullmove_number = fullmove.parse().expect("invalid fen: invalid fullmove number");

        chessboard
    }
}
