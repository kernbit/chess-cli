use anyhow::{anyhow, Result};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub file: u8,
    pub rank: u8,
}

impl Position {
    pub fn new(file: u8, rank: u8) -> Result<Self> {
        if file > 7 || rank > 7 {
            return Err(anyhow!("Invalid position: file {} rank {}", file, rank));
        }
        Ok(Position { file, rank })
    }
    
    pub fn from_algebraic(notation: &str) -> Result<Self> {
        if notation.len() != 2 {
            return Err(anyhow!("Invalid algebraic notation: {}", notation));
        }
        
        let chars: Vec<char> = notation.chars().collect();
        let file = match chars[0] {
            'a' => 0, 'b' => 1, 'c' => 2, 'd' => 3,
            'e' => 4, 'f' => 5, 'g' => 6, 'h' => 7,
            _ => return Err(anyhow!("Invalid file: {}", chars[0])),
        };
        
        let rank = match chars[1] {
            '1' => 0, '2' => 1, '3' => 2, '4' => 3,
            '5' => 4, '6' => 5, '7' => 6, '8' => 7,
            _ => return Err(anyhow!("Invalid rank: {}", chars[1])),
        };
        
        Ok(Position { file, rank })
    }
    
    pub fn to_algebraic(&self) -> String {
        let file_char = (b'a' + self.file) as char;
        let rank_char = (b'1' + self.rank) as char;
        format!("{}{}", file_char, rank_char)
    }
    
    pub fn is_valid(&self) -> bool {
        self.file <= 7 && self.rank <= 7
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_algebraic())
    }
}