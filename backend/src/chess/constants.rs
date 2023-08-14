use std::collections::HashMap;
use std::collections::hash_map::RandomState;

use crate::chess::enums::chess_color::ChessColor;

struct Constants {
    rook_line_mask_hashmap: HashMap<u64, u64>
}

impl Constants {
    pub fn new(&self) -> Self {
        Constants {
            rook_line_mask_hashmap: Self::make_rook_line_mask_hashmap()
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
    fn get_corrdinates_from_bit_position(bit_position: u64) -> (u64, u64) {
        (bit_position % 8, bit_position / 8)
    }

    fn get_bit_position_from_coordinates(col: u64, row: u64) -> u64 {
        row * 8 + col
    }

    fn make_rook_line_mask_hashmap() -> HashMap<u64, u64> {
        let mut rook_line_mask_hashmap: HashMap<u64, u64> = HashMap::with_hasher(Self::get_new_hash_builder());

        for i in 0..64 {
            rook_line_mask_hashmap.insert(i, Self::make_rook_line_mask_hashmap_helper(i));
        }

        rook_line_mask_hashmap
    }

    fn make_rook_line_mask_hashmap_helper(rook_position: u64) -> u64 {
        let (col, row) = Self::get_corrdinates_from_bit_position(rook_position);
        let mut res: u64 = 0;

        // Right
        for i in 0..col {
            res += 1 << Self::get_bit_position_from_coordinates(i, row);
        }

        // Left
        for i in (col + 1)..8 {
            res += 1 << Self::get_bit_position_from_coordinates(i, row);
        }

        // Down
        for i in 0..row {
            res += 1 << Self::get_bit_position_from_coordinates(col, i);
        }

        // Up
        for i in (row + 1)..8 {
            res += 1 << Self::get_bit_position_from_coordinates(col, i);
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

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
        assert_eq!(Constants::get_corrdinates_from_bit_position(0), (0, 0), "Bit position 0.");
        assert_eq!(Constants::get_corrdinates_from_bit_position(7), (7, 0), "Bit position 7.");
        assert_eq!(Constants::get_corrdinates_from_bit_position(14), (6, 1), "Bit position 14.");
        assert_eq!(Constants::get_corrdinates_from_bit_position(63), (7, 7), "Bit position 63.");
    }

    #[test]
    fn test_get_bit_position_from_coordinates() {
        assert_eq!(Constants::get_bit_position_from_coordinates(0, 0), 0, "Coordinates (0,0).");
        assert_eq!(Constants::get_bit_position_from_coordinates(7, 0), 7, "Coordinates (7,0).");
        assert_eq!(Constants::get_bit_position_from_coordinates(6, 1), 14, "Coordinates (6,1).");
        assert_eq!(Constants::get_bit_position_from_coordinates(7, 7), 63, "Coordinates (7,7).");
    }

    #[test]
    fn test_make_rook_line_mask_dict_helper() {
        assert_eq!(Constants::make_rook_line_mask_hashmap_helper(0), 72_340_172_838_076_926, "Rook bit position 0.");
        assert_eq!(Constants::make_rook_line_mask_hashmap_helper(7), 9_259_542_123_273_814_143, "Rook bit position 7.");
        assert_eq!(Constants::make_rook_line_mask_hashmap_helper(25), 144_680_349_887_234_562, "Rook bit position 25.");
    }
}
