#![allow(dead_code)]

use crate::chess::enums::{
    chess_piece::ChessPiece,
    chess_color::ChessColor
};

use std::collections::HashMap;
use crate::chess::constants::Constants;

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

    fn get_occupied_spaces(&self) -> u64 {
        let mut res = 0;

        for i in 0..2 {
            for j in 0..6 {
                res |= self.board[i][j];
            }
        }

        res
    }

    pub fn get_threatened_spaces(&self, chess_color: ChessColor, constants: &Constants) -> u64 {
        let mut res = 0;

        let middle_pawns = self.board[chess_color as usize][ChessPiece::Pawn as usize] & constants.pawn_middle_mask;
        let left_pawns = self.board[chess_color as usize][ChessPiece::Pawn as usize] & constants.pawn_left_mask;
        let right_pawns = self.board[chess_color as usize][ChessPiece::Pawn as usize] & constants.pawn_right_mask;

        // Pawns
        match chess_color {
            ChessColor::White => {
                // Middle pawns
                res |= middle_pawns << 7;
                res |= middle_pawns << 9;

                // Left pawns
                res |= left_pawns << 7;

                // Right pawns
                res |= right_pawns << 7;
            },
            ChessColor::Black => {
                // Middle pawns
                res |= middle_pawns >> 7;
                res |= middle_pawns >> 9;

                // Left pawns
                res |= left_pawns >> 7;

                // Right pawns
                res |= right_pawns >> 7;
            }
        }

        let occupied_spaces = self.get_occupied_spaces();

        // Rooks
        let rooks = &self.board[chess_color as usize][ChessPiece::Rook as usize];
        if rooks.count_ones() != 0 {
            let rook_bit_positions = if rooks.count_ones() > constants.num_to_bit_position_max_val as u32 {
                Constants::find_bit_positions_from_num(*rooks)
            } else {
                constants.num_to_bit_position_hashmap.get(rooks).unwrap().to_vec()
            };
    
            for bit_position in rook_bit_positions {
                let hashmap: &HashMap<(ChessColor, u64, u64), u64> = &constants.rook_threat_hashmap;
                let mask: &u64 = hashmap.get(&(chess_color, bit_position, 0)).unwrap();
                res |= hashmap.get(&(chess_color, bit_position, occupied_spaces & mask)).unwrap();
            }
        }
        

        // Bishops
        let bishops = &self.board[chess_color as usize][ChessPiece::Bishop as usize];
        if bishops.count_ones() != 0 {
            let bishop_bit_positions = if bishops.count_ones() > constants.num_to_bit_position_max_val as u32 {
                Constants::find_bit_positions_from_num(*bishops)
            } else {
                constants.num_to_bit_position_hashmap.get(bishops).unwrap().to_vec()
            };
    
            for bit_position in bishop_bit_positions {
                let hashmap: &HashMap<(ChessColor, u64, u64), u64> = &constants.bishop_threat_hashmap;
                let mask: &u64 = hashmap.get(&(chess_color, bit_position, 0)).unwrap();
                res |= hashmap.get(&(chess_color, bit_position, occupied_spaces & mask)).unwrap();
            }
        }

        // Knights
        let knights = &self.board[chess_color as usize][ChessPiece::Knight as usize];
        if knights.count_ones() != 0 {
            let knight_bit_positions = if knights.count_ones() > constants.num_to_bit_position_max_val as u32 {
                Constants::find_bit_positions_from_num(*knights)
            } else {
                constants.num_to_bit_position_hashmap.get(knights).unwrap().to_vec()
            };
    
            for bit_position in knight_bit_positions {
                let hashmap: &HashMap<(ChessColor, u64, u64), u64> = &constants.knight_threat_hashmap;
                let mask: &u64 = hashmap.get(&(chess_color, bit_position, 0)).unwrap();
                res |= hashmap.get(&(chess_color, bit_position, occupied_spaces & mask)).unwrap();
            }
        }

        // Queens
        let queens = &self.board[chess_color as usize][ChessPiece::Queen as usize];
        if queens.count_ones() != 0 {
            let queen_bit_positions = if queens.count_ones() > constants.num_to_bit_position_max_val as u32 {
                Constants::find_bit_positions_from_num(*queens)
            } else {
                constants.num_to_bit_position_hashmap.get(queens).unwrap().to_vec()
            };
    
            for bit_position in queen_bit_positions {
                let rook_hashmap: &HashMap<(ChessColor, u64, u64), u64> = &constants.rook_threat_hashmap;
                let bishop_hashmap: &HashMap<(ChessColor, u64, u64), u64> = &constants.bishop_threat_hashmap;
    
                let rook_mask: &u64 = rook_hashmap.get(&(chess_color, bit_position, 0)).unwrap();
                let bishop_mask: &u64 = bishop_hashmap.get(&(chess_color, bit_position, 0)).unwrap();
    
                res |= rook_hashmap.get(&(chess_color, bit_position, occupied_spaces & rook_mask)).unwrap();
                res |= bishop_hashmap.get(&(chess_color, bit_position, occupied_spaces & bishop_mask)).unwrap();
            }
        }

        // Kings
        let kings = &self.board[chess_color as usize][ChessPiece::King as usize];
        if kings.count_ones() != 0 {
            let king_bit_positions = if kings.count_ones() > constants.num_to_bit_position_max_val as u32 {
                Constants::find_bit_positions_from_num(*kings)
            } else {
                constants.num_to_bit_position_hashmap.get(kings).unwrap().to_vec()
            };
    
            for bit_position in king_bit_positions {
                let hashmap: &HashMap<(ChessColor, u64, u64), u64> = &constants.king_threat_hashmap;
                let mask: &u64 = hashmap.get(&(chess_color, bit_position, 0)).unwrap();
                res |= hashmap.get(&(chess_color, bit_position, occupied_spaces & mask)).unwrap();
            }
        }

        res
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

    #[test]
    fn test_get_threatened_spaces_pawn() {
        let constants: Constants = Constants::new();
        let mut bitboard: BitBoard = BitBoard::new();
        bitboard.board = [[0; 6]; 2];
        bitboard.board[ChessColor::White as usize][ChessPiece::Pawn as usize] = 21_504;

        assert_eq!(bitboard.get_threatened_spaces(ChessColor::White, &constants), 11_141_120);
        assert_eq!(bitboard.get_threatened_spaces(ChessColor::Black, &constants), 0);

        bitboard.board[ChessColor::White as usize][ChessPiece::Pawn as usize] = 0;
        bitboard.board[ChessColor::Black as usize][ChessPiece::Pawn as usize] = 21_504;

        assert_eq!(bitboard.get_threatened_spaces(ChessColor::White, &constants), 0);
        assert_eq!(bitboard.get_threatened_spaces(ChessColor::Black, &constants), 170);
    }

    #[test]
    fn test_get_threatened_spaces_rook() {
        let constants: Constants = Constants::new();
        let mut bitboard: BitBoard = BitBoard::new();
        bitboard.board = [[0; 6]; 2];
        bitboard.board[ChessColor::White as usize][ChessPiece::Rook as usize] = 35_184_376_284_160;

        assert_eq!(bitboard.get_threatened_spaces(ChessColor::White, &constants), 7_234_152_523_743_886_180);
        assert_eq!(bitboard.get_threatened_spaces(ChessColor::Black, &constants), 0);

        bitboard.board[ChessColor::White as usize][ChessPiece::Rook as usize] = 0;
        bitboard.board[ChessColor::Black as usize][ChessPiece::Rook as usize] = 35_184_376_284_160;

        assert_eq!(bitboard.get_threatened_spaces(ChessColor::White, &constants), 0);
        assert_eq!(bitboard.get_threatened_spaces(ChessColor::Black, &constants), 7_234_152_523_743_886_180);
    }

    #[test]
    fn test_get_threatened_spaces_bishop() {
        let constants: Constants = Constants::new();
        let mut bitboard: BitBoard = BitBoard::new();
        bitboard.board = [[0; 6]; 2];
        bitboard.board[ChessColor::White as usize][ChessPiece::Bishop as usize] = 35_184_376_284_160;

        assert_eq!(bitboard.get_threatened_spaces(ChessColor::White, &constants), 10_003_700_321_274_995_227);
        assert_eq!(bitboard.get_threatened_spaces(ChessColor::Black, &constants), 0);

        bitboard.board[ChessColor::White as usize][ChessPiece::Bishop as usize] = 0;
        bitboard.board[ChessColor::Black as usize][ChessPiece::Bishop as usize] = 35_184_376_284_160;

        assert_eq!(bitboard.get_threatened_spaces(ChessColor::White, &constants), 0);
        assert_eq!(bitboard.get_threatened_spaces(ChessColor::Black, &constants), 10_003_700_321_274_995_227);
    }

    #[test]
    fn test_get_threatened_spaces_knight() {
        let constants: Constants = Constants::new();
        let mut bitboard: BitBoard = BitBoard::new();
        bitboard.board = [[0; 6]; 2];
        bitboard.board[ChessColor::White as usize][ChessPiece::Knight as usize] = 35_184_376_284_160;

        assert_eq!(bitboard.get_threatened_spaces(ChessColor::White, &constants), 5_802_888_842_932_457_649);
        assert_eq!(bitboard.get_threatened_spaces(ChessColor::Black, &constants), 0);

        bitboard.board[ChessColor::White as usize][ChessPiece::Knight as usize] = 0;
        bitboard.board[ChessColor::Black as usize][ChessPiece::Knight as usize] = 35_184_376_284_160;

        assert_eq!(bitboard.get_threatened_spaces(ChessColor::White, &constants), 0);
        assert_eq!(bitboard.get_threatened_spaces(ChessColor::Black, &constants), 5_802_888_842_932_457_649);
    }

    #[test]
    fn test_get_threatened_spaces_queen() {
        let constants: Constants = Constants::new();
        let mut bitboard: BitBoard = BitBoard::new();
        bitboard.board = [[0; 6]; 2];
        bitboard.board[ChessColor::White as usize][ChessPiece::Queen as usize] = 35_184_372_089_856;

        assert_eq!(bitboard.get_threatened_spaces(ChessColor::White, &constants), 12_462_831_761_324_178_223);
        assert_eq!(bitboard.get_threatened_spaces(ChessColor::Black, &constants), 0);

        bitboard.board[ChessColor::White as usize][ChessPiece::Queen as usize] = 0;
        bitboard.board[ChessColor::Black as usize][ChessPiece::Queen as usize] = 35_184_372_089_856;

        assert_eq!(bitboard.get_threatened_spaces(ChessColor::White, &constants), 0);
        assert_eq!(bitboard.get_threatened_spaces(ChessColor::Black, &constants), 12_462_831_761_324_178_223);
    }

    #[test]
    fn test_get_threatened_spaces_king() {
        let constants: Constants = Constants::new();
        let mut bitboard: BitBoard = BitBoard::new();
        bitboard.board = [[0; 6]; 2];
        bitboard.board[ChessColor::White as usize][ChessPiece::King as usize] = 5_120;

        assert_eq!(bitboard.get_threatened_spaces(ChessColor::White, &constants), 4_074_046);
        assert_eq!(bitboard.get_threatened_spaces(ChessColor::Black, &constants), 0);

        bitboard.board[ChessColor::White as usize][ChessPiece::King as usize] = 0;
        bitboard.board[ChessColor::Black as usize][ChessPiece::King as usize] = 5_120;

        assert_eq!(bitboard.get_threatened_spaces(ChessColor::White, &constants), 0);
        assert_eq!(bitboard.get_threatened_spaces(ChessColor::Black, &constants), 4_074_046);
    }

    #[test]
    fn test_get_threatened_spaces_mixed() {
        let constants: Constants = Constants::new();
        let bitboard: BitBoard = BitBoard::new();

        assert_eq!(bitboard.get_threatened_spaces(ChessColor::White, &constants), 16_777_086);
        assert_eq!(bitboard.get_threatened_spaces(ChessColor::Black, &constants), 9_151_313_343_305_220_096);
    }
}

