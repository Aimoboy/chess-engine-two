
#[derive(Debug, Clone, Copy)]
pub enum ChessColor {
    White,
    Black
}

impl ChessColor {
    pub fn get_opposite_color(&self) -> Self {
        match &self {
            ChessColor::White => ChessColor::Black,
            ChessColor::Black => ChessColor::White
        }
    }
}
