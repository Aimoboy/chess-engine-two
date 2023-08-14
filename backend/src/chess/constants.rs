
struct Constants {

}

impl Constants {

    fn get_all_possible_combinations_of_bits(number: u64) -> Vec<u64> {
        if number == 0 {
            return vec![0];
        }

        let mut highest_bit: i32 = 0;
        for i in 1..64 {
            if (1 << i) & number > 0 {
                highest_bit = i;
            }
        }
        let highest_bit_num = 1 << highest_bit;

        let res: Vec<u64> = Self::get_all_possible_combinations_of_bits(number - highest_bit_num);
        let mut new_res = Vec::with_capacity(res.capacity() * 2);
        new_res.extend(res.clone());

        for num in res {
            new_res.push(num + highest_bit_num);
        }

        new_res
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
}
