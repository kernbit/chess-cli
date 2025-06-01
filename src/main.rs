use anyhow::{anyhow, Result};
use clap::Parser;
use crossterm::{
    cursor, execute,
    style::{Color, ResetColor, SetForegroundColor},
    terminal::{self, ClearType},
};
use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

mod chess;
mod engine;
mod ui;

use chess::{Board, GameState, Move, PieceType, Player, Position};
use engine::StockfishEngine;
use ui::ChessUI;

#[derive(Parser)]
#[command(name = "chess-cli")]
#[command(about = "A CLI chess game with Stockfish integration")]
struct Args {
    #[arg(short, long, default_value = "stockfish")]
    engine_path: String,
    
    #[arg(short, long, default_value = "1000")]
    time_limit: u64,
    
    #[arg(short, long)]
    player_white: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    execute!(io::stdout(), terminal::Clear(ClearType::All), cursor::MoveTo(0, 0))?;
    
    let result = run_game(args);
    
    execute!(io::stdout(), cursor::MoveTo(0, 0), terminal::Clear(ClearType::All))?;
    
    result
}

fn run_game(args: Args) -> Result<()> {
    let mut board = Board::new();
    let mut engine = StockfishEngine::new(&args.engine_path)?;
    let ui = ChessUI::new();
    
    let player_is_white = args.player_white;
    let time_limit = Duration::from_millis(args.time_limit);
    
    println!("🏰 Chess CLI - Playing against Stockfish");
    println!("Player: {} | Engine: {}", 
        if player_is_white { "White" } else { "Black" },
        if player_is_white { "Black" } else { "White" }
    );
    println!("Enter moves in algebraic notation (e.g., e4, Nf3, O-O)");
    println!("Type 'quit' to exit, 'help' for commands\n");
    
    loop {
        ui.display_board(&board)?;
        ui.display_game_info(&board)?;
        
        match board.game_state() {
            GameState::Checkmate => {
                let winner = if board.current_player() == Player::White { "Black" } else { "White" };
                println!("🏁 Checkmate! {} wins!", winner);
                break;
            }
            GameState::Stalemate => {
                println!("🤝 Stalemate! Game is a draw.");
                break;
            }
            GameState::Draw => {
                println!("🤝 Draw!");
                break;
            }
            GameState::InProgress => {}
        }
        
        let player_turn = (board.current_player() == Player::White) == player_is_white;
        
        if player_turn {
            match get_player_move(&mut board)? {
                Some(chess_move) => {
                    if board.is_legal_move(&chess_move) {
                        board.make_move(&chess_move)?;
                        println!("✓ Move played: {}", chess_move.to_algebraic());
                    } else {
                        println!("❌ Illegal move! Try again.");
                        continue;
                    }
                }
                None => break,
            }
        } else {
            println!("🤖 Stockfish is thinking...");
            
            let engine_move = engine.get_best_move(&board, time_limit)?;
            match engine_move {
                Some(chess_move) => {
                    board.make_move(&chess_move)?;
                    println!("🤖 Stockfish plays: {}", chess_move.to_algebraic());
                }
                None => {
                    println!("🤖 Stockfish couldn't find a move!");
                    break;
                }
            }
        }
        
        thread::sleep(Duration::from_millis(500));
    }
    
    println!("\nGame Over! Thanks for playing!");
    Ok(())
}

fn get_player_move(board: &Board) -> Result<Option<Move>> {
    loop {
        print!("Enter your move: ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        
        match input.to_lowercase().as_str() {
            "quit" | "exit" | "q" => return Ok(None),
            "help" | "h" => {
                print_help();
                continue;
            }
            "board" | "b" => continue,
            _ => {
                match parse_algebraic_notation(input, board) {
                    Ok(chess_move) => return Ok(Some(chess_move)),
                    Err(e) => {
                        println!("❌ Invalid move '{}': {}", input, e);
                        continue;
                    }
                }
            }
        }
    }
}

fn print_help() {
    println!("\n📖 Help:");
    println!("  • Enter moves in algebraic notation:");
    println!("    - Pawn moves: e4, d5, exd5");
    println!("    - Piece moves: Nf3, Bb5, Qh4");
    println!("    - Castling: O-O (kingside), O-O-O (queenside)");
    println!("    - Promotion: e8=Q");
    println!("  • Commands:");
    println!("    - help/h: Show this help");
    println!("    - quit/q: Quit game");
    println!("    - board/b: Redraw board\n");
}

fn parse_algebraic_notation(notation: &str, board: &Board) -> Result<Move> {
    let notation = notation.trim().to_lowercase();
    
    
    if notation == "o-o" || notation == "0-0" {
        let from = if board.current_player() == Player::White { 
            Position::from_algebraic("e1")? 
        } else { 
            Position::from_algebraic("e8")? 
        };
        let to = if board.current_player() == Player::White { 
            Position::from_algebraic("g1")? 
        } else { 
            Position::from_algebraic("g8")? 
        };
        return Ok(Move::new(from, to, None));
    }
    
    if notation == "o-o-o" || notation == "0-0-0" {
        let from = if board.current_player() == Player::White { 
            Position::from_algebraic("e1")? 
        } else { 
            Position::from_algebraic("e8")? 
        };
        let to = if board.current_player() == Player::White { 
            Position::from_algebraic("c1")? 
        } else { 
            Position::from_algebraic("c8")? 
        };
        return Ok(Move::new(from, to, None));
    }
    
    
    if notation.len() == 2 {
        let to_pos = Position::from_algebraic(&notation)?;
        
        // Piyon hamlesi olarak varsay
        let from_rank = if board.current_player() == Player::White {
            if to_pos.rank == 3 { 1 } else { to_pos.rank - 1 }
        } else {
            if to_pos.rank == 4 { 6 } else { to_pos.rank + 1 }
        };
        
        let from_pos = Position::new(to_pos.file, from_rank)?;
        return Ok(Move::new(from_pos, to_pos, None));
    }
    
    
    if notation.len() == 4 {
        let from_pos = Position::from_algebraic(&notation[0..2])?;
        let to_pos = Position::from_algebraic(&notation[2..4])?;
        return Ok(Move::new(from_pos, to_pos, None));
    }
    
    Err(anyhow!("Could not parse move: '{}'. Try format like 'e4' or 'e2e4'", notation))
}
