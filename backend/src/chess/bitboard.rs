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
        board[ChessPiece::BlackKing as usize] += 1 << 51;

        BitBoard {
            board
        }
    }
}
