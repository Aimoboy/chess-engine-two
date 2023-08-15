#![allow(dead_code)]

use std::collections::HashMap;
use std::collections::hash_map::RandomState;

struct Constants {
    rook_line_mask_hashmap: HashMap<u64, u64>, // Input: position, Output: line mask
    rook_threaten_hashmap: HashMap<(u64, u64), u64>, // Input: (position, pieces on lines), Output: threatened spaces
    bishop_line_mask_hashmap: HashMap<u64, u64>, // Input: position, Output: line mask
    bishop_threaten_hashmap: HashMap<(u64, u64), u64>, // Input: (position, pieces on lines), Output: threatened spaces
}

impl Constants {
    pub fn new(&self) -> Self {
        Constants {
            rook_line_mask_hashmap: Self::make_rook_line_mask_hashmap(),
            rook_threaten_hashmap: Self::make_rook_threaten_hashmap(),
            bishop_line_mask_hashmap: Self::make_bishop_line_mask_hashmap(),
            bishop_threaten_hashmap: Self::make_bishop_threaten_hashmap()
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

    fn make_rook_line_mask_hashmap() -> HashMap<u64, u64> {
        let mut rook_line_mask_hashmap: HashMap<u64, u64> = HashMap::with_hasher(Self::get_new_hash_builder());

        for i in 0..64 {
            rook_line_mask_hashmap.insert(i, Self::make_rook_threaten_hashmap_helper(i, 0));
        }

        rook_line_mask_hashmap
    }

    fn make_rook_threaten_hashmap() -> HashMap<(u64, u64), u64> {
        let mut rook_threaten_hashmap: HashMap<(u64, u64), u64> = HashMap::with_hasher(Self::get_new_hash_builder());

        for position in 0..64 {
            let line_mask = Self::make_rook_threaten_hashmap_helper(position, 0);
            let other_piece_combinations: Vec<u64> = Self::get_all_possible_combinations_of_bits(line_mask);

            for combination in other_piece_combinations {
                rook_threaten_hashmap.insert((position, combination), Self::make_rook_threaten_hashmap_helper(position, combination));
            }
        }

        rook_threaten_hashmap
    }

    fn make_rook_threaten_hashmap_helper(rook_position: u64, other_pieces: u64) -> u64 {
        let (col, row) = Self::get_coordinates_from_bit_position(rook_position);
        let mut res = 0;

        // Right
        for i in (0..col).rev() {
            let num = Self::get_bit_position_from_coordinates(i, row);
            let num = 1 << num;
            res += num;

            if num & other_pieces > 0 {
                break;
            }
        }

        // Left
        for i in (col + 1)..8 {
            let num = Self::get_bit_position_from_coordinates(i, row);
            let num = 1 << num;
            res += num;

            if num & other_pieces > 0 {
                break;
            }
        }

        // Down
        for i in (0..row).rev() {
            let num = Self::get_bit_position_from_coordinates(col, i);
            let num = 1 << num;
            res += num;

            if num & other_pieces > 0 {
                break;
            }
        }

        // Up
        for i in (row + 1)..8 {
            let num = Self::get_bit_position_from_coordinates(col, i);
            let num = 1 << num;
            res += num;

            if num & other_pieces > 0 {
                break;
            }
        }

        res
    }

    fn make_bishop_line_mask_hashmap() -> HashMap<u64, u64> {
        let mut bishop_line_mask_hashmap: HashMap<u64, u64> = HashMap::with_hasher(Self::get_new_hash_builder());

        for i in 0..64 {
            bishop_line_mask_hashmap.insert(i, Self::make_bishop_threaten_hashmap_helper(i, 0));
        }

        bishop_line_mask_hashmap
    }

    fn make_bishop_threaten_hashmap() -> HashMap<(u64, u64), u64> {
        let mut bishop_threaten_hashmap: HashMap<(u64, u64), u64> = HashMap::with_hasher(Self::get_new_hash_builder());

        for position in 0..64 {
            let line_mask = Self::make_bishop_threaten_hashmap_helper(position, 0);
            let other_piece_combinations: Vec<u64> = Self::get_all_possible_combinations_of_bits(line_mask);

            for combination in other_piece_combinations {
                bishop_threaten_hashmap.insert((position, combination), Self::make_bishop_threaten_hashmap_helper(position, combination));
            }
        }

        bishop_threaten_hashmap
    }

    fn make_bishop_threaten_hashmap_helper(bishop_position: u64, other_pieces: u64) -> u64 {
        let (col, row) = Self::get_coordinates_from_bit_position(bishop_position);
        let mut res = 0;
        
        // Upper-Right
        for i in 1..std::cmp::min(col, 8 - row) {
            let num = Self::get_bit_position_from_coordinates(col - i, row + i);
            let num = 1 << num;
            res += num;

            if num & other_pieces > 0 {
                break;
            }
        }

        // Lower-Right
        for i in 1..std::cmp::min(col + 1, row + 1) {
            let num = Self::get_bit_position_from_coordinates(col - i, row - i);
            let num = 1 << num;
            res += num;

            if num & other_pieces > 0 {
                break;
            }
        }

        // Lower-Left
        for i in 1..std::cmp::min(8 - col, row + 1) {
            let num = Self::get_bit_position_from_coordinates(col + i, row - i);
            let num = 1 << num;
            res += num;

            if num & other_pieces > 0 {
                break;
            }
        }

        // Upper-Left
        for i in 1..std::cmp::min(8 - col, 8 - row) {
            let num = Self::get_bit_position_from_coordinates(col + i, row + i);
            let num = 1 << num;
            res += num;

            if num & other_pieces > 0 {
                break;
            }
        }
        
        res
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
    fn testmake_rook_threaten_hashmap_helper() {
        // No other pieces on the lines
        assert_eq!(Constants::make_rook_threaten_hashmap_helper(0, 0), 72_340_172_838_076_926, "Rook bit position 0, no other pieces.");
        assert_eq!(Constants::make_rook_threaten_hashmap_helper(7, 0), 9_259_542_123_273_814_143, "Rook bit position 7, no other pieces.");
        assert_eq!(Constants::make_rook_threaten_hashmap_helper(25, 0), 144_680_349_887_234_562, "Rook bit position 25, no other pieces.");

        // Other pieces on the lines
        assert_eq!(Constants::make_rook_threaten_hashmap_helper(28, 17_592_253_153_280), 17_664_865_996_816, "Rook bit position 28, other pieces.");
        assert_eq!(Constants::make_rook_threaten_hashmap_helper(4, 4_503_599_627_370_497), 4_521_260_802_380_015, "Rook bit position 4, other pieces.");
    }

    #[test]
    fn testmake_rook_threaten_hashmap() {
        let hashmap: HashMap<(u64, u64), u64> = Constants::make_rook_threaten_hashmap();

        // No other pieces on the lines
        assert_eq!(*hashmap.get(&(0, 0)).unwrap(), 72_340_172_838_076_926, "Rook bit position 0, no other pieces.");
        assert_eq!(*hashmap.get(&(7, 0)).unwrap(), 9_259_542_123_273_814_143, "Rook bit position 7, no other pieces.");
        assert_eq!(*hashmap.get(&(25, 0)).unwrap(), 144_680_349_887_234_562, "Rook bit position 25, no other pieces.");

        // Other pieces on the lines
        assert_eq!(*hashmap.get(&(28, 17_592_253_153_280)).unwrap(), 17_664_865_996_816, "Rook bit position 28, other pieces.");
        assert_eq!(*hashmap.get(&(4, 4_503_599_627_370_497)).unwrap(), 4_521_260_802_380_015, "Rook bit position 4, other pieces.");
    }

    #[test]
    fn testmake_bishop_threaten_hashmap_helper() {
        // No other pieces on the lines
        assert_eq!(Constants::make_bishop_threaten_hashmap_helper(0, 0), 9_241_421_688_590_303_744, "Bishop bit position 0, no other pieces.");
        assert_eq!(Constants::make_bishop_threaten_hashmap_helper(63, 0), 18_049_651_735_527_937, "Bishop bit position 63, no other pieces.");

        // Other pieces on the lines
        assert_eq!(Constants::make_bishop_threaten_hashmap_helper(12, 67_108_864), 550_899_286_056, "Bishop bit position 12, other pieces.");
    }

    #[test]
    fn testmake_bishop_threaten_hashmap() {
        let hashmap: HashMap<(u64, u64), u64> = Constants::make_bishop_threaten_hashmap();

        // No other pieces on the lines
        assert_eq!(*hashmap.get(&(0, 0)).unwrap(), 9_241_421_688_590_303_744, "Bishop bit position 0, no other pieces.");
        assert_eq!(*hashmap.get(&(63, 0)).unwrap(), 18_049_651_735_527_937, "Bishop bit position 63, no other pieces.");

        // Other pieces on the lines
        assert_eq!(*hashmap.get(&(12, 67_108_864)).unwrap(), 550_899_286_056, "Bishop bit position 12, other pieces.");
    }
}
