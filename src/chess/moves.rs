use super::{PieceType, Position};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Move {
    from: Position,
    to: Position,
    promotion: Option<PieceType>,
    is_capture: bool,
    is_castling: bool,
    is_en_passant: bool,
}

impl Move {
    pub fn new(from: Position, to: Position, promotion: Option<PieceType>) -> Self {
        Move {
            from,
            to,
            promotion,
            is_capture: false,
            is_castling: false,
            is_en_passant: false,
        }
    }
    
    pub fn with_capture(mut self) -> Self {
        self.is_capture = true;
        self
    }
    
    pub fn with_castling(mut self) -> Self {
        self.is_castling = true;
        self
    }
    
    pub fn with_en_passant(mut self) -> Self {
        self.is_en_passant = true;
        self
    }
    
    pub fn from(&self) -> Position {
        self.from
    }
    
    pub fn to(&self) -> Position {
        self.to
    }
    
    pub fn promotion(&self) -> Option<PieceType> {
        self.promotion
    }
    
    pub fn to_algebraic(&self) -> String {
        if self.is_castling {
            if self.to.file == 6 {
                return "O-O".to_string();
            } else {
                return "O-O-O".to_string();
            }
        }
        
        let mut result = String::new();
        result.push_str(&self.to.to_algebraic());
        
        if let Some(promotion_piece) = self.promotion {
            result.push('=');
            result.push(match promotion_piece {
                PieceType::Queen => 'Q',
                PieceType::Rook => 'R',
                PieceType::Bishop => 'B',
                PieceType::Knight => 'N',
                _ => 'Q',
            });
        }
        
        result
    }
    
    pub fn to_uci(&self) -> String {
        let mut result = format!("{}{}", self.from.to_algebraic(), self.to.to_algebraic());
        
        if let Some(promotion_piece) = self.promotion {
            result.push(match promotion_piece {
                PieceType::Queen => 'q',
                PieceType::Rook => 'r',
                PieceType::Bishop => 'b',
                PieceType::Knight => 'n',
                _ => 'q',
            });
        }
        
        result
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_algebraic())
    }
}