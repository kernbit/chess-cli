use super::{Move, Piece, PieceType, Player, Position};
use anyhow::{anyhow, Result};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameState {
    InProgress,
    Checkmate,
    Stalemate,
    Draw,
}

#[derive(Debug, Clone)]
pub struct Board {
    pieces: HashMap<Position, Piece>,
    current_player: Player,
    move_count: u32,
    halfmove_clock: u32,
    castling_rights: CastlingRights,
    en_passant_target: Option<Position>,
}

#[derive(Debug, Clone)]
struct CastlingRights {
    white_kingside: bool,
    white_queenside: bool,
    black_kingside: bool,
    black_queenside: bool,
}

impl Board {
    pub fn new() -> Self {
        let mut board = Board {
            pieces: HashMap::new(),
            current_player: Player::White,
            move_count: 1,
            halfmove_clock: 0,
            castling_rights: CastlingRights {
                white_kingside: true,
                white_queenside: true,
                black_kingside: true,
                black_queenside: true,
            },
            en_passant_target: None,
        };
        
        board.setup_initial_position();
        board
    }
    
    fn setup_initial_position(&mut self) {
        self.pieces.insert(Position::new(0, 0).unwrap(), Piece::new(PieceType::Rook, Player::White));
        self.pieces.insert(Position::new(1, 0).unwrap(), Piece::new(PieceType::Knight, Player::White));
        self.pieces.insert(Position::new(2, 0).unwrap(), Piece::new(PieceType::Bishop, Player::White));
        self.pieces.insert(Position::new(3, 0).unwrap(), Piece::new(PieceType::Queen, Player::White));
        self.pieces.insert(Position::new(4, 0).unwrap(), Piece::new(PieceType::King, Player::White));
        self.pieces.insert(Position::new(5, 0).unwrap(), Piece::new(PieceType::Bishop, Player::White));
        self.pieces.insert(Position::new(6, 0).unwrap(), Piece::new(PieceType::Knight, Player::White));
        self.pieces.insert(Position::new(7, 0).unwrap(), Piece::new(PieceType::Rook, Player::White));
        
        for file in 0..8 {
            self.pieces.insert(Position::new(file, 1).unwrap(), Piece::new(PieceType::Pawn, Player::White));
        }
        
        self.pieces.insert(Position::new(0, 7).unwrap(), Piece::new(PieceType::Rook, Player::Black));
        self.pieces.insert(Position::new(1, 7).unwrap(), Piece::new(PieceType::Knight, Player::Black));
        self.pieces.insert(Position::new(2, 7).unwrap(), Piece::new(PieceType::Bishop, Player::Black));
        self.pieces.insert(Position::new(3, 7).unwrap(), Piece::new(PieceType::Queen, Player::Black));
        self.pieces.insert(Position::new(4, 7).unwrap(), Piece::new(PieceType::King, Player::Black));
        self.pieces.insert(Position::new(5, 7).unwrap(), Piece::new(PieceType::Bishop, Player::Black));
        self.pieces.insert(Position::new(6, 7).unwrap(), Piece::new(PieceType::Knight, Player::Black));
        self.pieces.insert(Position::new(7, 7).unwrap(), Piece::new(PieceType::Rook, Player::Black));
        
        for file in 0..8 {
            self.pieces.insert(Position::new(file, 6).unwrap(), Piece::new(PieceType::Pawn, Player::Black));
        }
    }
    
    pub fn piece_at(&self, position: Position) -> Option<&Piece> {
        self.pieces.get(&position)
    }
    
    pub fn current_player(&self) -> Player {
        self.current_player
    }
    
    pub fn move_count(&self) -> u32 {
        self.move_count
    }
    
    pub fn make_move(&mut self, chess_move: &Move) -> Result<()> {
        if !self.is_legal_move(chess_move) {
            return Err(anyhow!("Illegal move"));
        }
        
        if let Some(piece) = self.pieces.remove(&chess_move.from()) {
            self.pieces.insert(chess_move.to(), piece);
        }
        
        if let Some(promotion_type) = chess_move.promotion() {
            if let Some(piece) = self.pieces.get_mut(&chess_move.to()) {
                piece.piece_type = promotion_type;
            }
        }
        
        self.current_player = self.current_player.opposite();
        if self.current_player == Player::White {
            self.move_count += 1;
        }
        
        Ok(())
    }
    
    pub fn is_legal_move(&self, chess_move: &Move) -> bool {
        if let Some(piece) = self.piece_at(chess_move.from()) {
            if piece.player != self.current_player {
                return false;
            }
            
            if let Some(dest_piece) = self.piece_at(chess_move.to()) {
                if dest_piece.player == piece.player {
                    return false;
                }
            }
            
            return true;
        }
        
        false
    }
    
    pub fn get_legal_moves(&self) -> Vec<Move> {
        vec![]
    }
    
    pub fn game_state(&self) -> GameState {
        GameState::InProgress
    }
    
    pub fn is_in_check(&self, _player: Player) -> bool {
        false
    }
    
    pub fn to_fen(&self) -> String {
        let mut fen = String::new();
        
        for rank in (0..8).rev() {
            let mut empty_count = 0;
            for file in 0..8 {
                let pos = Position::new(file, rank).unwrap();
                if let Some(piece) = self.piece_at(pos) {
                    if empty_count > 0 {
                        fen.push_str(&empty_count.to_string());
                        empty_count = 0;
                    }
                    let symbol = match (piece.piece_type, piece.player) {
                        (PieceType::King, Player::White) => 'K',
                        (PieceType::Queen, Player::White) => 'Q',
                        (PieceType::Rook, Player::White) => 'R',
                        (PieceType::Bishop, Player::White) => 'B',
                        (PieceType::Knight, Player::White) => 'N',
                        (PieceType::Pawn, Player::White) => 'P',
                        (PieceType::King, Player::Black) => 'k',
                        (PieceType::Queen, Player::Black) => 'q',
                        (PieceType::Rook, Player::Black) => 'r',
                        (PieceType::Bishop, Player::Black) => 'b',
                        (PieceType::Knight, Player::Black) => 'n',
                        (PieceType::Pawn, Player::Black) => 'p',
                    };
                    fen.push(symbol);
                } else {
                    empty_count += 1;
                }
            }
            if empty_count > 0 {
                fen.push_str(&empty_count.to_string());
            }
            if rank > 0 {
                fen.push('/');
            }
        }
        
        fen.push(' ');
        fen.push(if self.current_player == Player::White { 'w' } else { 'b' });
        
        fen.push(' ');
        let mut castling = String::new();
        if self.castling_rights.white_kingside { castling.push('K'); }
        if self.castling_rights.white_queenside { castling.push('Q'); }
        if self.castling_rights.black_kingside { castling.push('k'); }
        if self.castling_rights.black_queenside { castling.push('q'); }
        if castling.is_empty() { castling.push('-'); }
        fen.push_str(&castling);
        
        fen.push(' ');
        if let Some(ep_target) = self.en_passant_target {
            fen.push_str(&ep_target.to_algebraic());
        } else {
            fen.push('-');
        }
        
        fen.push_str(&format!(" {} {}", self.halfmove_clock, self.move_count));
        
        fen
    }
}