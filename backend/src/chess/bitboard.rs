#![allow(dead_code)]

use crate::chess::enums::{
    chess_piece::ChessPiece,
    chess_color::ChessColor
};

struct BitBoard {
    board: [[u64; 6]; 2]
}

impl BitBoard {
    pub fn new() -> Self {
        let mut board: [[u64; 6]; 2] = [[0; 6]; 2];

        // White pawns
        for i in 8..16 {
            board[ChessColor::White as usize][ChessPiece::Pawn as usize] += 1 << i;
        }

        // Black pawns
        for i in 48..56 {
            board[ChessColor::Black as usize][ChessPiece::Pawn as usize] += 1 << i;
        }

        // White rooks
        board[ChessColor::White as usize][ChessPiece::Rook as usize] += 1 << 0;
        board[ChessColor::White as usize][ChessPiece::Rook as usize] += 1 << 7;

        // Black rooks
        board[ChessColor::Black as usize][ChessPiece::Rook as usize] += 1 << 56;
        board[ChessColor::Black as usize][ChessPiece::Rook as usize] += 1 << 63;

        // White knights
        board[ChessColor::White as usize][ChessPiece::Knight as usize] += 1 << 1;
        board[ChessColor::White as usize][ChessPiece::Knight as usize] += 1 << 6;

        // Black knights
        board[ChessColor::Black as usize][ChessPiece::Knight as usize] += 1 << 57;
        board[ChessColor::Black as usize][ChessPiece::Knight as usize] += 1 << 62;

        // White bishops
        board[ChessColor::White as usize][ChessPiece::Bishop as usize] += 1 << 2;
        board[ChessColor::White as usize][ChessPiece::Bishop as usize] += 1 << 5;

        // Black bishops
        board[ChessColor::Black as usize][ChessPiece::Bishop as usize] += 1 << 58;
        board[ChessColor::Black as usize][ChessPiece::Bishop as usize] += 1 << 61;

        // Queens
        board[ChessColor::White as usize][ChessPiece::Queen as usize] += 1 << 4;
        board[ChessColor::Black as usize][ChessPiece::Queen as usize] += 1 << 60;

        // Kings
        board[ChessColor::White as usize][ChessPiece::King as usize] += 1 << 3;
        board[ChessColor::Black as usize][ChessPiece::King as usize] += 1 << 59;

        BitBoard {
            board
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitboard_start_board() {
        let board: [[u64; 6]; 2] = BitBoard::new().board;

        assert_eq!(board[ChessColor::White as usize][ChessPiece::Pawn as usize], 65_280, "White pawns.");
        assert_eq!(board[ChessColor::Black as usize][ChessPiece::Pawn as usize], 71_776_119_061_217_280, "Black pawns.");

        assert_eq!(board[ChessColor::White as usize][ChessPiece::Rook as usize], 129, "White rooks.");
        assert_eq!(board[ChessColor::Black as usize][ChessPiece::Rook as usize], 9_295_429_630_892_703_744, "Black rooks.");

        assert_eq!(board[ChessColor::White as usize][ChessPiece::Knight as usize], 66, "White knights.");
        assert_eq!(board[ChessColor::Black as usize][ChessPiece::Knight as usize], 4_755_801_206_503_243_776, "Black knights.");

        assert_eq!(board[ChessColor::White as usize][ChessPiece::Bishop as usize], 36, "White bishops.");
        assert_eq!(board[ChessColor::Black as usize][ChessPiece::Bishop as usize], 2_594_073_385_365_405_696, "Black bishops.");

        assert_eq!(board[ChessColor::White as usize][ChessPiece::Queen as usize], 16, "White queen.");
        assert_eq!(board[ChessColor::Black as usize][ChessPiece::Queen as usize], 1_152_921_504_606_846_976, "Black queen.");

        assert_eq!(board[ChessColor::White as usize][ChessPiece::King as usize], 8, "White king.");
        assert_eq!(board[ChessColor::Black as usize][ChessPiece::King as usize], 576_460_752_303_423_488, "Black king.");

        assert_eq!(
            board[0][0] + board[0][1] + board[0][2] + board[0][3] + board[0][4] + board[0][5] + board[1][0] + board[1][1] + board[1][2] + board[1][3] + board[1][4] + board[1][5],
            board[0][0] | board[0][1] | board[0][2] | board[0][3] | board[0][4] | board[0][5] | board[1][0] | board[1][1] | board[1][2] | board[1][3] + board[1][4] | board[1][5],
            "Check for overlapping pieces"
        )
    }
}

