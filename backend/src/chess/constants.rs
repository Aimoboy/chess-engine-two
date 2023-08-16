#![allow(dead_code)]

use std::collections::HashMap;
use std::collections::hash_map::RandomState;

pub struct Constants {
    pub rook_attack_mask_hashmap: HashMap<u64, u64>, // Input: position, Output: attack mask
    pub rook_threat_hashmap: HashMap<(u64, u64), u64>, // Input: (position, pieces on possibly threatened pieces), Output: threatened spaces
    pub bishop_attack_mask_hashmap: HashMap<u64, u64>, // Input: position, Output: attack mask
    pub bishop_threat_hashmap: HashMap<(u64, u64), u64>, // Input: (position, pieces on possibly threatened pieces), Output: threatened spaces
    pub knight_attack_mask_hashmap: HashMap<u64, u64>, // Input: position, Output: attack mask
    pub knight_threat_hashmap: HashMap<(u64, u64), u64>, // Input: (position, pieces on possibly threatened pieces), Output: threatened spaces
    pub king_attack_mask_hashmap: HashMap<u64, u64>, // Input: position, Output: attack mask
    pub king_threat_hashmap: HashMap<(u64, u64), u64>, // Input: (position, pieces on possibly threatened pieces), Output: threatened spaces
    pub pawn_middle_mask: u64,
    pub pawn_left_mask: u64,
    pub pawn_right_mask: u64,
    pub num_to_bit_position_hashmap: HashMap<u64, Vec<u64>>,
    pub num_to_bit_position_max_val: u64
}

const PAWN_MIDDLE_MASK: u64 = 9_114_861_777_597_660_798;
const PAWN_LEFT_MASK: u64 = 9_259_542_123_273_814_144;
const PAWN_RIGHT_MASK: u64 = 72_340_172_838_076_673;

const NUM_TO_BIT_POSITION_MAX_VAL: u64 = 3;

impl Constants {
    pub fn new() -> Self {
        Constants {
            rook_attack_mask_hashmap: Self::make_attack_mask_hashmap(Self::get_rook_moves()),
            rook_threat_hashmap: Self::make_threat_hashmap(Self::get_rook_moves()),
            bishop_attack_mask_hashmap: Self::make_attack_mask_hashmap(Self::get_bishop_moves()),
            bishop_threat_hashmap: Self::make_threat_hashmap(Self::get_bishop_moves()),
            knight_attack_mask_hashmap: Self::make_attack_mask_hashmap(Self::get_knight_moves()),
            knight_threat_hashmap: Self::make_threat_hashmap(Self::get_knight_moves()),
            king_attack_mask_hashmap: Self::make_attack_mask_hashmap(Self::get_king_moves()),
            king_threat_hashmap: Self::make_threat_hashmap(Self::get_king_moves()),
            pawn_middle_mask: PAWN_MIDDLE_MASK,
            pawn_left_mask: PAWN_LEFT_MASK,
            pawn_right_mask: PAWN_RIGHT_MASK,
            num_to_bit_position_hashmap: Self::make_num_to_bit_positions_hashmap(NUM_TO_BIT_POSITION_MAX_VAL),
            num_to_bit_position_max_val: NUM_TO_BIT_POSITION_MAX_VAL
        }
    }

    fn get_all_possible_combinations_of_bits(number: u64) -> Vec<u64> {
        let mut res: Vec<u64> = Vec::with_capacity(2_usize.pow(number.count_ones()));

        Self::get_all_possible_combinations_of_bits_helper(number, &mut res);

        res
    }

    fn get_all_possible_combinations_of_bits_helper(number: u64, vec: &mut Vec<u64>) {
        if number == 0 {
            vec.push(0);
            return;
        }

        let highest_bit = 63 - number.leading_zeros();
        let highest_bit_num = 1 << highest_bit;

        Self::get_all_possible_combinations_of_bits_helper(number - highest_bit_num, vec);
        
        let length = vec.len();
        for i in 0..length {
            vec.push(vec[i] + highest_bit_num);
        }
    }

    fn get_new_hash_builder() -> RandomState {
        RandomState::new()
    }

    // The coordinates are (col, row) and the positions start from the lower right as 0 to the upper left as 63
    fn get_coordinates_from_bit_position(bit_position: u64) -> (u64, u64) {
        (bit_position % 8, bit_position / 8)
    }

    fn get_bit_position_from_coordinates(col: u64, row: u64) -> u64 {
        row * 8 + col
    }

    fn make_attack_mask_hashmap(piece_moves: Vec<Vec<(i32, i32)>>) -> HashMap<u64, u64> {
        let piece_threat_generator = Self::threat_generator(piece_moves);
        let mut attack_mask_hashmap: HashMap<u64, u64> = HashMap::with_hasher(Self::get_new_hash_builder());

        for i in 0..64 {
            attack_mask_hashmap.insert(i, piece_threat_generator(i, 0));
        }

        attack_mask_hashmap
    }

    fn make_threat_hashmap(piece_moves: Vec<Vec<(i32, i32)>>) -> HashMap<(u64, u64), u64> {
        let piece_threat_generator = Self::threat_generator(piece_moves);
        let mut threat_hashmap: HashMap<(u64, u64), u64> = HashMap::with_hasher(Self::get_new_hash_builder());

        for position in 0..64 {
            let attack_mask = piece_threat_generator(position, 0);
            let other_piece_combinations: Vec<u64> = Self::get_all_possible_combinations_of_bits(attack_mask);

            for combination in other_piece_combinations {
                threat_hashmap.insert((position, combination), piece_threat_generator(position, combination));
            }
        }

        threat_hashmap
    }

    fn threat_generator(piece_moves: Vec<Vec<(i32, i32)>>) -> impl Fn(u64, u64) -> u64 {
        Box::new(move |piece_postition: u64, other_pieces: u64| {
            let (col, row) = Self::get_coordinates_from_bit_position(piece_postition);
            let col = col as i32;
            let row = row as i32;

            let mut res = 0;
            for move_set in piece_moves.iter() {
                for (col_rel, row_rel) in move_set {
                    let col_sum = col + col_rel;
                    let row_sum = row + row_rel;

                    if col_sum < 0 || col_sum > 7 {
                        break;
                    }
        
                    if row_sum < 0 || row_sum > 7 {
                        break;
                    }
        
                    let position = 1 << Self::get_bit_position_from_coordinates(col_sum as u64, row_sum as u64);
                    res += position;

                    if position & other_pieces > 0 {
                        break;
                    }
                }
            }

            res
        })
    }

    fn get_rook_moves() -> Vec<Vec<(i32, i32)>> {
        vec![
            vec![(1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0)], // Left
            vec![(-1, 0), (-2, 0), (-3, 0), (-4, 0), (-5, 0), (-6, 0), (-7, 0)], // Right
            vec![(0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (0, 6), (0, 7)], // Up
            vec![(0, -1), (0, -2), (0, -3), (0, -4), (0, -5), (0, -6), (0, -7)] // Down
        ]
    }

    fn get_bishop_moves() -> Vec<Vec<(i32, i32)>> {
        vec![
            vec![(1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6), (7, 7)], // Upper-Left
            vec![(-1, 1), (-2, 2), (-3, 3), (-4, 4), (-5, 5), (-6, 6), (-7, 7)], // Upper-Right
            vec![(1, -1), (2, -2), (3, -3), (4, -4), (5, -5), (6, -6), (7, -7)], // Lower-Left
            vec![(-1, -1), (-2, -2), (-3, -3), (-4, -4), (-5, -5), (-6, -6), (-7, -7)] // Lower-Right
        ]
    }

    fn get_knight_moves() -> Vec<Vec<(i32, i32)>> {
        vec![
            vec![(1, 2)],
            vec![(2, 1)],
            vec![(-1, -2)],
            vec![(-2, -1)],
            vec![(-1, 2)],
            vec![(-2, 1)],
            vec![(1, -2)],
            vec![(2, -1)]
        ]
    }

    fn get_king_moves() -> Vec<Vec<(i32, i32)>> {
        vec![
            vec![(1, 1)],
            vec![(0, 1)],
            vec![(-1, 1)],
            vec![(-1, 0)],
            vec![(-1, -1)],
            vec![(0, -1)],
            vec![(1, -1)],
            vec![(1, 0)]
        ]
    }

    fn make_num_to_bit_positions_hashmap(max_positions: u64) -> HashMap<u64, Vec<u64>> {
        let mut all_bit_positions: Vec<u64> = Vec::new();

        for i in 1..(max_positions + 1) {
            all_bit_positions.append(&mut Self::board_choose_n_all_combinations(i));
        }

        let mut hashmap: HashMap<u64, Vec<u64>> = HashMap::with_hasher(Self::get_new_hash_builder());
        for bit_position in all_bit_positions {
            hashmap.insert(bit_position, Self::find_bit_positions_from_num(bit_position));
        }

        hashmap
    }

    fn find_bit_positions_from_num(num: u64) -> Vec<u64> {
        let mut num = num;
        let mut res: Vec<u64> = Vec::new();

        while num > 0 {
            let pos = num.trailing_zeros() as u64;
            res.push(pos);
            num -= 1 << pos;
        }

        res
    }

    fn board_choose_n_all_combinations(num: u64) -> Vec<u64> {
        if num < 1 {
            panic!()
        }

        if num == 1 {
            let mut res = Vec::with_capacity(64);
            for i in 0..64 {
                res.push(1 << i);
            }
            return res;
        }

        let rec_res: Vec<u64> = Self::board_choose_n_all_combinations(num - 1);
        let mut new_res: Vec<u64> = Vec::new();

        for res in rec_res {
            for i in 0..64 {
                let tmp: u64 = res + (1 << i);

                if tmp.count_ones() < num as u32 {
                    continue;
                }

                if new_res.contains(&tmp) {
                    continue;
                }

                new_res.push(tmp);
            }
        }

        new_res
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};
    use super::*;

    #[test]
    fn test_get_all_possible_combinations_of_bits_0() {
        let res: Vec<u64> = Constants::get_all_possible_combinations_of_bits(0);
        assert_eq!(res.len(), 1, "There should be exactly 1 item in the vector.");

        let hashset: HashSet<u64> = HashSet::from_iter(res);
        assert!(hashset.contains(&0), "The vector should contain the item 0.");
    }

    #[test]
    fn test_get_all_possible_combinations_of_bits_1() {
        let res: Vec<u64> = Constants::get_all_possible_combinations_of_bits(1);
        assert_eq!(res.len(), 2, "There should be exactly 2 items in the vector.");

        let hashset: HashSet<u64> = HashSet::from_iter(res);
        assert!(hashset.contains(&0), "The vector should contain the item 0.");
        assert!(hashset.contains(&1), "The vector should contain the item 1.");
    }

    #[test]
    fn test_get_all_possible_combinations_of_bits_7() {
        let res: Vec<u64> = Constants::get_all_possible_combinations_of_bits(7);
        assert_eq!(res.len(), 8, "There should be exactly 8 items in the vector.");

        let hashset: HashSet<u64> = HashSet::from_iter(res);
        assert!(hashset.contains(&0), "The vector should contain the item 0.");
        assert!(hashset.contains(&1), "The vector should contain the item 1.");
        assert!(hashset.contains(&2), "The vector should contain the item 2.");
        assert!(hashset.contains(&3), "The vector should contain the item 3.");
        assert!(hashset.contains(&4), "The vector should contain the item 4.");
        assert!(hashset.contains(&5), "The vector should contain the item 5.");
        assert!(hashset.contains(&6), "The vector should contain the item 6.");
        assert!(hashset.contains(&7), "The vector should contain the item 7.");
    }

    #[test]
    fn test_get_all_possible_combinations_of_bits_288() {
        let res: Vec<u64> = Constants::get_all_possible_combinations_of_bits(288);
        assert_eq!(res.len(), 4, "There should be exactly 4 items in the vector.");

        let hashset: HashSet<u64> = HashSet::from_iter(res);
        assert!(hashset.contains(&0), "The vector should contain the item 0.");
        assert!(hashset.contains(&32), "The vector should contain the item 32.");
        assert!(hashset.contains(&256), "The vector should contain the item 256.");
        assert!(hashset.contains(&288), "The vector should contain the item 288.");
    }

    #[test]
    fn test_get_coordinates_from_bit_position() {
        assert_eq!(Constants::get_coordinates_from_bit_position(0), (0, 0), "Bit position 0.");
        assert_eq!(Constants::get_coordinates_from_bit_position(7), (7, 0), "Bit position 7.");
        assert_eq!(Constants::get_coordinates_from_bit_position(14), (6, 1), "Bit position 14.");
        assert_eq!(Constants::get_coordinates_from_bit_position(63), (7, 7), "Bit position 63.");
    }

    #[test]
    fn test_get_bit_position_from_coordinates() {
        assert_eq!(Constants::get_bit_position_from_coordinates(0, 0), 0, "Coordinates (0,0).");
        assert_eq!(Constants::get_bit_position_from_coordinates(7, 0), 7, "Coordinates (7,0).");
        assert_eq!(Constants::get_bit_position_from_coordinates(6, 1), 14, "Coordinates (6,1).");
        assert_eq!(Constants::get_bit_position_from_coordinates(7, 7), 63, "Coordinates (7,7).");
    }

    #[test]
    fn test_rook_threat_generator() {
        let rook_threat_generator = Constants::threat_generator(Constants::get_rook_moves());

        // No other pieces
        assert_eq!(rook_threat_generator(0, 0), 72_340_172_838_076_926, "Rook bit position 0, no other pieces.");
        assert_eq!(rook_threat_generator(7, 0), 9_259_542_123_273_814_143, "Rook bit position 7, no other pieces.");
        assert_eq!(rook_threat_generator(25, 0), 144_680_349_887_234_562, "Rook bit position 25, no other pieces.");

        // Other pieces
        assert_eq!(rook_threat_generator(28, 17_592_253_153_280), 17_664_865_996_816, "Rook bit position 28, other pieces.");
        assert_eq!(rook_threat_generator(4, 4_503_599_627_370_497), 4_521_260_802_380_015, "Rook bit position 4, other pieces.");
    }

    #[test]
    fn test_make_rook_threat_hashmap() {
        let hashmap: HashMap<(u64, u64), u64> = Constants::make_threat_hashmap(Constants::get_rook_moves());

        // No other pieces
        assert_eq!(*hashmap.get(&(0, 0)).unwrap(), 72_340_172_838_076_926, "Rook bit position 0, no other pieces.");
        assert_eq!(*hashmap.get(&(7, 0)).unwrap(), 9_259_542_123_273_814_143, "Rook bit position 7, no other pieces.");
        assert_eq!(*hashmap.get(&(25, 0)).unwrap(), 144_680_349_887_234_562, "Rook bit position 25, no other pieces.");

        // Other pieces
        assert_eq!(*hashmap.get(&(28, 17_592_253_153_280)).unwrap(), 17_664_865_996_816, "Rook bit position 28, other pieces.");
        assert_eq!(*hashmap.get(&(4, 4_503_599_627_370_497)).unwrap(), 4_521_260_802_380_015, "Rook bit position 4, other pieces.");
    }

    #[test]
    fn test_bishop_threat_generator() {
        let bishop_threat_generator = Constants::threat_generator(Constants::get_bishop_moves());

        // No other pieces
        assert_eq!(bishop_threat_generator(0, 0), 9_241_421_688_590_303_744, "Bishop bit position 0, no other pieces.");
        assert_eq!(bishop_threat_generator(63, 0), 18_049_651_735_527_937, "Bishop bit position 63, no other pieces.");

        // Other pieces
        assert_eq!(bishop_threat_generator(12, 67_108_864), 550_899_286_056, "Bishop bit position 12, other pieces.");
        assert_eq!(bishop_threat_generator(14, 34_368_126_976), 34_638_659_744, "Bishop bit position 14 other pieces.");
    }

    #[test]
    fn test_make_bishop_threat_hashmap() {
        let hashmap: HashMap<(u64, u64), u64> = Constants::make_threat_hashmap(Constants::get_bishop_moves());

        // No other pieces
        assert_eq!(*hashmap.get(&(0, 0)).unwrap(), 9_241_421_688_590_303_744, "Bishop bit position 0, no other pieces.");
        assert_eq!(*hashmap.get(&(63, 0)).unwrap(), 18_049_651_735_527_937, "Bishop bit position 63, no other pieces.");

        // Other pieces
        assert_eq!(*hashmap.get(&(12, 67_108_864)).unwrap(), 550_899_286_056, "Bishop bit position 12, other pieces.");
        assert_eq!(*hashmap.get(&(14, 34_368_126_976)).unwrap(), 34_638_659_744, "Bishop bit position 14, other pieces.");
    }

    #[test]
    fn test_knight_threat_generator() {
        let knight_threat_generator = Constants::threat_generator(Constants::get_knight_moves());

        // No other pieces
        assert_eq!(knight_threat_generator(0, 0), 132_096, "Knight bit position 0, no other pieces.");
        assert_eq!(knight_threat_generator(28, 0), 44_272_527_353_856, "Knight bit position 28, no other pieces.");

        // Other pieces
        assert_eq!(knight_threat_generator(0, 131_072), 132_096, "Knight bit position 0, other pieces.");
        assert_eq!(knight_threat_generator(28, 43_980_469_315_584), 44_272_527_353_856, "Knight bit position 28, other pieces.");
    }

    #[test]
    fn test_make_knight_threat_hashmap() {
        let hashmap: HashMap<(u64, u64), u64> = Constants::make_threat_hashmap(Constants::get_knight_moves());

        // No other pieces
        assert_eq!(*hashmap.get(&(0, 0)).unwrap(), 132_096, "Knight bit position 0, no other pieces.");
        assert_eq!(*hashmap.get(&(28, 0)).unwrap(), 44_272_527_353_856, "Knight bit position 28, no other pieces.");

        // Other pieces
        assert_eq!(*hashmap.get(&(0, 131_072)).unwrap(), 132_096, "Knight bit position 0, other pieces.");
        assert_eq!(*hashmap.get(&(28, 43_980_469_315_584)).unwrap(), 44_272_527_353_856, "Knight bit position 28, other pieces.");
    }

    #[test]
    fn test_king_threat_generator() {
        let king_threat_generator = Constants::threat_generator(Constants::get_king_moves());

        // No other pieces
        assert_eq!(king_threat_generator(0, 0), 770, "King bit position 0, no other pieces.");
        assert_eq!(king_threat_generator(14, 0), 14_721_248, "King bit position 14, no other pieces.");

        // Other pieces
        assert_eq!(king_threat_generator(0, 2), 770, "King bit position 0, other pieces.");
        assert_eq!(king_threat_generator(14, 10_526_720), 14_721_248, "King bit position 0, other pieces.");
    }

    #[test]
    fn test_make_king_threat_hashmap() {
        let hashmap: HashMap<(u64, u64), u64> = Constants::make_threat_hashmap(Constants::get_king_moves());

        // No other pieces
        assert_eq!(*hashmap.get(&(0, 0)).unwrap(), 770, "King bit position 0, no other pieces.");
        assert_eq!(*hashmap.get(&(14, 0)).unwrap(), 14_721_248, "King bit position 14, no other pieces.");

        // Other pieces
        assert_eq!(*hashmap.get(&(0, 2)).unwrap(), 770, "King bit position 0, other pieces.");
        assert_eq!(*hashmap.get(&(14, 10_526_720)).unwrap(), 14_721_248, "King bit position 0, other pieces.");
    }

    #[test]
    fn test_board_choose_n_all_combinations_1() {
        let res: Vec<u64> = Constants::board_choose_n_all_combinations(1);

        assert_eq!(res.len(), 64, "Correct length check.");

        let mut res_clone = res.clone();
        res_clone.sort();
        res_clone.dedup();
        assert_eq!(res_clone.len(), res.len(), "No duplicate check");

        for item in res {
            assert_eq!(item.count_ones(), 1, "Correct number of ones check.");
        }
    }

    #[test]
    fn test_board_choose_n_all_combinations_2() {
        let res: Vec<u64> = Constants::board_choose_n_all_combinations(2);

        assert_eq!(res.len(), 2_016, "Correct length check.");

        let mut res_clone = res.clone();
        res_clone.sort();
        res_clone.dedup();
        assert_eq!(res_clone.len(), res.len(), "No duplicate check");

        for item in res {
            assert_eq!(item.count_ones(), 2, "Correct number of ones check.");
        }
    }

    #[test]
    fn test_board_choose_n_all_combinations_3() {
        let res: Vec<u64> = Constants::board_choose_n_all_combinations(3);

        assert_eq!(res.len(), 41_664, "Correct length check.");

        let mut res_clone = res.clone();
        res_clone.sort();
        res_clone.dedup();
        assert_eq!(res_clone.len(), res.len(), "No duplicate check");

        for item in res {
            assert_eq!(item.count_ones(), 3, "Correct number of ones check.");
        }
    }

    #[test]
    fn test_find_bit_positions_from_num() {
        let num = (1 << 10) + (1 << 5) + (1 << 60);
        let res = Constants::find_bit_positions_from_num(num);

        assert_eq!(res.len(), 3, "Should contain 3 items.");
        assert!(res.contains(&10), "Should contain 10.");
        assert!(res.contains(&5), "Should contain 5.");
        assert!(res.contains(&60), "Should contain 60.");
    }



    #[test]
    fn test_make_num_to_bit_positions_hashmap() {
        let hashmap: HashMap<u64, Vec<u64>> = Constants::make_num_to_bit_positions_hashmap(3);

        let num = (1 << 5) + (1 << 8) + (1 << 55);
        let res = hashmap.get(&num).unwrap();

        assert_eq!(res.len(), 3, "Should contain 3 items.");
        assert!(res.contains(&5), "Should contain 5.");
        assert!(res.contains(&8), "Should contain 8.");
        assert!(res.contains(&55), "Should contain 55.");

        assert!(!hashmap.contains_key(&0), "Should not contain 0.");

        let num = (1 << 0) + (1 << 15);
        let res = hashmap.get(&num).unwrap();

        assert_eq!(res.len(), 2, "Should contain 2 items.");
        assert!(res.contains(&0), "Should contain 0.");
        assert!(res.contains(&15), "Should contain 15.");
    }
}
