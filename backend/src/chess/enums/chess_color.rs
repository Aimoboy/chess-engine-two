
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ChessColor {
    White = 0,
    Black = 1
}

impl ChessColor {
    pub fn get_opposite_color(&self) -> Self {
        match &self {
            ChessColor::White => ChessColor::Black,
            ChessColor::Black => ChessColor::White
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_opposite_color() {
        assert_eq!(ChessColor::White, ChessColor::Black.get_opposite_color(), "The opposite of black should be white.");
        assert_eq!(ChessColor::Black, ChessColor::White.get_opposite_color(), "The opposite of white should be black.");
    }
}
