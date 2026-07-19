use std::collections::{HashMap, HashSet};

pub struct Solution;

impl Solution {
    pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
        if arr.is_empty() {
            return vec![];
        }

        let mut sorted_unique: Vec<i32> = arr
            .iter()
            .copied()
            .collect::<HashSet<i32>>()
            .into_iter()
            .collect();
        sorted_unique.sort();

        let rank_map: HashMap<i32, i32> = sorted_unique
            .into_iter()
            .enumerate()
            .map(|(rank, value)| (value, rank as i32 + 1))
            .collect();

        arr.into_iter().map(|value| rank_map[&value]).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leetcode_example_1() {
        assert_eq!(
            Solution::array_rank_transform(vec![40, 10, 20, 30]),
            vec![4, 1, 2, 3]
        )
    }

    #[test]
    fn test_leetcode_example_2() {
        assert_eq!(
            Solution::array_rank_transform(vec![100, 100, 100]),
            vec![1, 1, 1]
        )
    }

    #[test]
    fn test_leetcode_example_3() {
        assert_eq!(
            Solution::array_rank_transform(vec![37, 12, 28, 9, 100, 56, 80, 5, 12]),
            vec![5, 3, 4, 2, 8, 6, 7, 1, 3]
        )
    }
}
