use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    White,
    Black,
}

impl Player {
    pub fn opposite(&self) -> Player {
        match self {
            Player::White => Player::Black,
            Player::Black => Player::White,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Piece {
    pub piece_type: PieceType,
    pub player: Player,
}

impl Piece {
    pub fn new(piece_type: PieceType, player: Player) -> Self {
        Piece { piece_type, player }
    }
    
    pub fn unicode_symbol(&self) -> char {
        match (self.player, self.piece_type) {
            (Player::White, PieceType::King) => '♔',
            (Player::White, PieceType::Queen) => '♕',
            (Player::White, PieceType::Rook) => '♖',
            (Player::White, PieceType::Bishop) => '♗',
            (Player::White, PieceType::Knight) => '♘',
            (Player::White, PieceType::Pawn) => '♙',
            (Player::Black, PieceType::King) => '♚',
            (Player::Black, PieceType::Queen) => '♛',
            (Player::Black, PieceType::Rook) => '♜',
            (Player::Black, PieceType::Bishop) => '♝',
            (Player::Black, PieceType::Knight) => '♞',
            (Player::Black, PieceType::Pawn) => '♟',
        }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.unicode_symbol())
    }
}