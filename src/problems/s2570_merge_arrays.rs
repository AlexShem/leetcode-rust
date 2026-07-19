use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut output = nums1
            .into_iter()
            .chain(nums2.into_iter())
            .fold(HashMap::new(), |mut acc, pair| {
                acc.entry(pair[0]).and_modify(|v| *v += pair[1]).or_insert(pair[1]);
                acc
            })
            .into_iter()
            .map(|(k, v)| vec![k, v])
            .collect::<Vec<Vec<i32>>>();
        output.sort();
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leetcode_example_1() {
        let nums1 = vec![vec![1, 2], vec![2, 3], vec![4, 5]];
        let nums2 = vec![vec![1, 4], vec![3, 2], vec![4, 1]];
        let expected = vec![vec![1, 6], vec![2, 3], vec![3, 2], vec![4, 6]];
        assert_eq!(Solution::merge_arrays(nums1, nums2), expected)
    }

    #[test]
    fn test_leetcode_example_2() {
        let nums1 = vec![vec![2, 4], vec![3, 6], vec![5, 5]];
        let nums2 = vec![vec![1, 3], vec![4, 3]];
        let expected = vec![vec![1, 3], vec![2, 4], vec![3, 6], vec![4, 3], vec![5, 5]];
        assert_eq!(Solution::merge_arrays(nums1, nums2), expected)
    }
}
