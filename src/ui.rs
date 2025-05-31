use crate::chess::{Board, GameState, Player};
use anyhow::Result;
use crossterm::{
    cursor, execute,
    style::{Color, ResetColor, SetForegroundColor},
    terminal::{self, ClearType},
};
use std::io;

pub struct ChessUI;

impl ChessUI {
    pub fn new() -> Self {
        ChessUI
    }
    
    pub fn display_board(&self, board: &Board) -> Result<()> {
        execute!(io::stdout(), cursor::MoveTo(0, 0), terminal::Clear(ClearType::All))?;
        
        println!("┌───┬───┬───┬───┬───┬───┬───┬───┐");
        
        for rank in (0..8).rev() {
            print!("│");
            for file in 0..8 {
                let pos = crate::chess::Position::new(file, rank)?;
                let piece_char = if let Some(piece) = board.piece_at(pos) {
                    piece.unicode_symbol()
                } else {
                    ' '
                };
                
                let is_light_square = (file + rank) % 2 == 0;
                if is_light_square {
                    execute!(io::stdout(), SetForegroundColor(Color::White))?;
                } else {
                    execute!(io::stdout(), SetForegroundColor(Color::Yellow))?;
                }
                
                print!(" {} ", piece_char);
                execute!(io::stdout(), ResetColor)?;
                print!("│");
            }
            println!(" {}", rank + 1);
            
            if rank > 0 {
                println!("├───┼───┼───┼───┼───┼───┼───┼───┤");
            }
        }
        
        println!("└───┴───┴───┴───┴───┴───┴───┴───┘");
        println!("  a   b   c   d   e   f   g   h  ");
        println!();
        
        Ok(())
    }
    
    pub fn display_game_info(&self, board: &Board) -> Result<()> {
        let current_player = match board.current_player() {
            Player::White => "White",
            Player::Black => "Black",
        };
        
        println!("Turn: {} | Move: {}", current_player, board.move_count());
        
        if board.is_in_check(board.current_player()) {
            execute!(io::stdout(), SetForegroundColor(Color::Red))?;
            println!("⚠️  CHECK!");
            execute!(io::stdout(), ResetColor)?;
        }
        
        match board.game_state() {
            GameState::Checkmate => {
                execute!(io::stdout(), SetForegroundColor(Color::Red))?;
                println!("🏁 CHECKMATE!");
                execute!(io::stdout(), ResetColor)?;
            }
            GameState::Stalemate => {
                execute!(io::stdout(), SetForegroundColor(Color::Yellow))?;
                println!("🤝 STALEMATE!");
                execute!(io::stdout(), ResetColor)?;
            }
            GameState::Draw => {
                execute!(io::stdout(), SetForegroundColor(Color::Yellow))?;
                println!("🤝 DRAW!");
                execute!(io::stdout(), ResetColor)?;
            }
            GameState::InProgress => {}
        }
        
        println!();
        Ok(())
    }
}