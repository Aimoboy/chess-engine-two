#![allow(dead_code)]

use crate::chess::enums::chess_piece::ChessPiece;

struct BitBoard {
    board: [u64; 12]
}

impl BitBoard {
    pub fn new() -> Self {
        let mut board: [u64; 12] = [0; 12];

        // White pawns
        for i in 8..16 {
            board[ChessPiece::WhitePawn as usize] += 1 << i;
        }

        // Black pawns
        for i in 48..56 {
            board[ChessPiece::BlackPawn as usize] += 1 << i;
        }

        // White rooks
        board[ChessPiece::WhiteRook as usize] += 1 << 0;
        board[ChessPiece::WhiteRook as usize] += 1 << 7;

        // Black rooks
        board[ChessPiece::BlackRook as usize] += 1 << 56;
        board[ChessPiece::BlackRook as usize] += 1 << 63;

        // White knights
        board[ChessPiece::WhiteKnight as usize] += 1 << 1;
        board[ChessPiece::WhiteKnight as usize] += 1 << 6;

        // Black knights
        board[ChessPiece::BlackKnight as usize] += 1 << 57;
        board[ChessPiece::BlackKnight as usize] += 1 << 62;

        // White bishops
        board[ChessPiece::WhiteBishop as usize] += 1 << 2;
        board[ChessPiece::WhiteBishop as usize] += 1 << 5;

        // Black bishops
        board[ChessPiece::BlackBishop as usize] += 1 << 58;
        board[ChessPiece::BlackBishop as usize] += 1 << 61;

        // Queens
        board[ChessPiece::WhiteQueen as usize] += 1 << 4;
        board[ChessPiece::BlackQueen as usize] += 1 << 60;

        // Kings
        board[ChessPiece::WhiteKing as usize] += 1 << 3;
        board[ChessPiece::BlackKing as usize] += 1 << 59;

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
        let board: [u64; 12] = BitBoard::new().board;

        assert_eq!(board[ChessPiece::WhitePawn as usize], 65_280, "White pawns.");
        assert_eq!(board[ChessPiece::BlackPawn as usize], 71_776_119_061_217_280, "Black pawns.");

        assert_eq!(board[ChessPiece::WhiteRook as usize], 129, "White rooks.");
        assert_eq!(board[ChessPiece::BlackRook as usize], 9_295_429_630_892_703_744, "Black rooks.");

        assert_eq!(board[ChessPiece::WhiteKnight as usize], 66, "White knights.");
        assert_eq!(board[ChessPiece::BlackKnight as usize], 4_755_801_206_503_243_776, "Black knights.");

        assert_eq!(board[ChessPiece::WhiteBishop as usize], 36, "White bishops.");
        assert_eq!(board[ChessPiece::BlackBishop as usize], 2_594_073_385_365_405_696, "Black bishops.");

        assert_eq!(board[ChessPiece::WhiteQueen as usize], 16, "White queen.");
        assert_eq!(board[ChessPiece::BlackQueen as usize], 1_152_921_504_606_846_976, "Black queen.");

        assert_eq!(board[ChessPiece::WhiteKing as usize], 8, "White king.");
        assert_eq!(board[ChessPiece::BlackKing as usize], 576_460_752_303_423_488, "Black king.");

        assert_eq!(
            board[0] + board[1] + board[2] + board[3] + board[4] + board[5] + board[6] + board[7] + board[8] + board[9] + board[10] + board[11],
            board[0] | board[1] | board[2] | board[3] | board[4] | board[5] | board[6] | board[7] | board[8] | board[9] | board[10] | board[11],
            "Check for overlapping pieces"
        )
    }
}

