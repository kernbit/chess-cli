use crate::chess::{Board, Move, Position};
use anyhow::{anyhow, Result};
use std::io::{BufRead, BufReader, Write};
use std::process::{Child, Command, Stdio};
use std::time::Duration;

pub struct StockfishEngine {
    process: Child,
}

impl StockfishEngine {
    pub fn new(engine_path: &str) -> Result<Self> {
        let process = Command::new(engine_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| anyhow!("Failed to start Stockfish: {}", e))?;
        
        let mut engine = StockfishEngine { process };
        
        engine.send_command("uci")?;
        engine.wait_for_response("uciok")?;
        engine.send_command("isready")?;
        engine.wait_for_response("readyok")?;
        
        Ok(engine)
    }
    
    pub fn get_best_move(&mut self, board: &Board, time_limit: Duration) -> Result<Option<Move>> {
        let fen = board.to_fen();
        
        self.send_command(&format!("position fen {}", fen))?;
        
        let time_ms = time_limit.as_millis();
        self.send_command(&format!("go movetime {}", time_ms))?;
        
        let best_move = self.read_best_move()?;
        
        if let Some(uci_move) = best_move {
            return Ok(Some(self.parse_uci_move(&uci_move)?));
        }
        
        Ok(None)
    }
    
    fn send_command(&mut self, command: &str) -> Result<()> {
        if let Some(ref mut stdin) = self.process.stdin {
            writeln!(stdin, "{}", command)?;
            stdin.flush()?;
        }
        Ok(())
    }
    
    fn wait_for_response(&mut self, expected: &str) -> Result<()> {
        if let Some(ref mut stdout) = self.process.stdout {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                let line = line?;
                if line.contains(expected) {
                    return Ok(());
                }
            }
        }
        Err(anyhow!("Expected response '{}' not received", expected))
    }
    
    fn read_best_move(&mut self) -> Result<Option<String>> {
        if let Some(ref mut stdout) = self.process.stdout {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                let line = line?;
                if line.starts_with("bestmove") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 2 && parts[1] != "(none)" {
                        return Ok(Some(parts[1].to_string()));
                    }
                    return Ok(None);
                }
            }
        }
        Ok(None)
    }
    
    fn parse_uci_move(&self, uci_move: &str) -> Result<Move> {
        if uci_move.len() < 4 {
            return Err(anyhow!("Invalid UCI move: {}", uci_move));
        }
        
        let from = Position::from_algebraic(&uci_move[0..2])?;
        let to = Position::from_algebraic(&uci_move[2..4])?;
        
        let promotion = if uci_move.len() == 5 {
            use crate::chess::PieceType;
            match uci_move.chars().nth(4).unwrap() {
                'q' => Some(PieceType::Queen),
                'r' => Some(PieceType::Rook),
                'b' => Some(PieceType::Bishop),
                'n' => Some(PieceType::Knight),
                _ => None,
            }
        } else {
            None
        };
        
        Ok(Move::new(from, to, promotion))
    }
}

impl Drop for StockfishEngine {
    fn drop(&mut self) {
        let _ = self.send_command("quit");
        let _ = self.process.wait();
    }
}
